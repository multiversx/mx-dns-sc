#include "multiversx/context.h"
#include "multiversx/util.h"

#define NULL 0
#define MIN_NAME_LENGTH 10
#define true 1
#define false 0

typedef int bool;
typedef byte ADDRESS[32];
typedef byte HASH[32];
typedef byte PAYMENT[32];

typedef enum
{
    None,
    Pending,
    Commited
} ValueState;

typedef struct
{
    ValueState state;
    ADDRESS address;
} Value;

void _validateName(const byte *name, int len);
void _hashName(const byte *name, int nameLen, HASH result);
byte _shardId(const ADDRESS address);
byte _getOwnShardId();
void _copy(byte *dest, const byte *source, int len);
void _copyRange(byte *dest, const byte *src, int destStart, int srcStart, int len);
bool _equal(const byte *op1, const byte *op2, int len);
void _loadValue(const HASH nameHash, Value *value);
void _storeValue(const HASH nameHash, const Value *value);
void _loadCallbackArg(HASH nameHash);
void _storeCallbackArg(const HASH nameHash);
void _clearCallbackArg();
void _resolveFromHash(const HASH nameHash, ADDRESS result);
void _constructKey(const byte *prefix, int prefixLen, const byte *arg, int argLen, byte *key);
int _constructAsyncCallData(const byte *funcName, int funcLen,
                            const byte *arg, int argLen, byte *data);
byte _halfByteToHexDigit(byte num);
void _hexEncode(const byte *data, int dataLen, byte *result);

const ADDRESS ZERO_32_BYTE_ARRAY = {0};

GENERAL_MSG(SET_USER_NAME_FUNCTION, "SetUserName");
GENERAL_MSG(CLAIM_MSG, "dns claim");

ERROR_MSG(ERR_NAME_TOO_SHORT, "name is too short");
ERROR_MSG(ERR_CHARACTER_NOT_ALLOWED, "character not allowed");
ERROR_MSG(ERR_WRONG_FEE, "should pay exactly the registration cost");
ERROR_MSG(ERR_DIFFERENT_SHARD, "name belongs to another shard");
ERROR_MSG(ERR_NAME_ALREADY_TAKEN, "name already taken");
ERROR_MSG(ERR_CLAIM, "only owner can claim");

// full keys
STORAGE_KEY(REGISTRATION_COST);

// partial keys
STORAGE_KEY(VALUE_STATE); // + HASH nameHash -> ValueState
STORAGE_KEY(CALLBACK);    // + HASH originalTxHash -> HASH nameHash

// endpoints

// Args:
// bigInt registration cost
void init()
{
    CHECK_NUM_ARGS(1);
    CHECK_NOT_PAYABLE();

    bigInt registrationCost = bigIntNew(0);
    bigIntGetUnsignedArgument(0, registrationCost);
    bigIntStorageStoreUnsigned(REGISTRATION_COST_KEY, REGISTRATION_COST_KEY_LEN,
                               registrationCost);
}

// PAYABLE
// Args:
// byte *name
void registerNameEndpoint()
{
    CHECK_NUM_ARGS(1);

    PAYMENT paymentAsBytes = {};
    bigInt payment = bigIntNew(0);
    bigInt registrationCost = bigIntNew(0);

    byte name[100] = {};
    int nameLen;
    HASH nameHash = {};

    Value value = {};
    ADDRESS callerAddress = {};

    byte dataAsync[200] = {};
    int dataLen;

    getCallValue(paymentAsBytes);
    bigIntSetUnsignedBytes(payment, paymentAsBytes, sizeof(PAYMENT));
    bigIntStorageLoadUnsigned(REGISTRATION_COST_KEY, REGISTRATION_COST_KEY_LEN,
                              registrationCost);

    if (bigIntCmp(payment, registrationCost) != 0)
    {
        SIGNAL_ERROR(ERR_WRONG_FEE);
    }

    nameLen = getArgument(0, name);
    _validateName(name, nameLen);
    _hashName(name, nameLen, nameHash);
    if (_shardId(nameHash) != _getOwnShardId())
    {
        SIGNAL_ERROR(ERR_DIFFERENT_SHARD);
    }

    _loadValue(nameHash, &value);
    if (value.state != None)
    {
        SIGNAL_ERROR(ERR_NAME_ALREADY_TAKEN);
    }

    getCaller(callerAddress);
    value.state = Pending;
    _copy(value.address, callerAddress, sizeof(ADDRESS));
    _storeValue(nameHash, &value);

    _storeCallbackArg(nameHash);

    dataLen = _constructAsyncCallData(SET_USER_NAME_FUNCTION, SET_USER_NAME_FUNCTION_LEN,
                                      name, nameLen, dataAsync);

    asyncCall(callerAddress, ZERO_32_BYTE_ARRAY, dataAsync, dataLen);
}

void claim()
{
    CHECK_NUM_ARGS(0);
    CHECK_NOT_PAYABLE();

    ADDRESS scAddress = {};
    ADDRESS contractOwner = {};
    ADDRESS caller = {};
    byte balance[32] = {};

    getOwnerAddress(contractOwner);
    getCaller(caller);
    if (!_equal(contractOwner, caller, sizeof(ADDRESS)))
    {
        SIGNAL_ERROR(ERR_CLAIM);
    }

    getSCAddress(scAddress);
    getExternalBalance(scAddress, balance);
    transferValue(contractOwner, balance, CLAIM_MSG, CLAIM_MSG_LEN);
}

// view functions

void getOwnerAddressView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(0);

    ADDRESS owner = {};
    getOwnerAddress(owner);

    finish(owner, sizeof(ADDRESS));
}

void getOwnShardIdView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(0);

    byte shardId = _getOwnShardId();

    finish(&shardId, sizeof(byte));
}

// Args:
// byte *name
void nameHashView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    int len;
    byte name[100] = {};
    HASH hash = {};

    len = getArgument(0, name);
    _hashName(name, len, hash);

    finish(hash, sizeof(HASH));
}

// Args:
// byte *name
void nameShardView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    int len;
    byte name[100] = {};
    HASH hash = {};
    byte shardId;

    len = getArgument(0, name);
    _hashName(name, len, hash);
    shardId = _shardId(hash);

    finish(&shardId, sizeof(byte));
}

// Args:
// byte *name
void validateNameView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    int len;
    byte name[100] = {};

    len = getArgument(0, name);
    _validateName(name, len);
}

// Args:
// byte *name
void resolveView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    byte name[100] = {};
    int len;
    HASH nameHash = {};
    ADDRESS addr = {0};

    len = getArgument(0, name);
    _hashName(name, len, nameHash);
    _resolveFromHash(nameHash, addr);

    finish(addr, sizeof(ADDRESS));
}

// Args:
// HASH *nameHash
void resolveFromHashView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    HASH nameHash = {};
    ADDRESS addr = {0};

    getArgument(0, nameHash);
    _resolveFromHash(nameHash, addr);

    finish(addr, sizeof(ADDRESS));
}

// private helpers

int _checkNameChar(byte ch)
{
    return (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'z');
}

void _validateName(const byte *name, int len)
{
    int i;

    if (len < MIN_NAME_LENGTH)
    {
        SIGNAL_ERROR(ERR_NAME_TOO_SHORT);
    }

    for (i = 0; i < len; i++)
    {
        if (!_checkNameChar(name[i]))
        {
            SIGNAL_ERROR(ERR_CHARACTER_NOT_ALLOWED);
        }
    }
}

void _hashName(const byte *name, int nameLen, HASH result)
{
    keccak256(name, nameLen, result);
}

byte _shardId(const ADDRESS address)
{
    return address[sizeof(ADDRESS) - 1];
}

byte _getOwnShardId()
{
    ADDRESS scAddr = {};
    getSCAddress(scAddr);

    return _shardId(scAddr);
}

void _copy(byte *dest, const byte *src, int len)
{
    int i;
    for (i = 0; i < len; i++)
    {
        dest[i] = src[i];
    }
}

void _copyRange(byte *dest, const byte *src, int destStart, int srcStart, int len)
{
    _copy(dest + destStart, src + srcStart, len);
}

bool _equal(const byte *op1, const byte *op2, int len)
{
    int i;
    for (i = 0; i < len; i++)
    {
        if (op1[i] != op2[i])
        {
            return false;
        }
    }

    return true;
}

void _loadValue(const HASH nameHash, Value *value)
{
    const int keyLen = VALUE_STATE_KEY_LEN + sizeof(HASH);
    byte key[keyLen] = {};

    _constructKey(VALUE_STATE_KEY, VALUE_STATE_KEY_LEN, nameHash, sizeof(HASH), key);
    storageLoad(key, keyLen, (byte *)value);
}

void _storeValue(const HASH nameHash, const Value *value)
{
    const int keyLen = VALUE_STATE_KEY_LEN + sizeof(HASH);
    byte key[keyLen] = {};

    _constructKey(VALUE_STATE_KEY, VALUE_STATE_KEY_LEN, nameHash, sizeof(HASH), key);
    storageStore(key, keyLen, (byte *)value, sizeof(Value));
}

void _loadCallbackArg(HASH arg)
{
    const int keyLen = CALLBACK_KEY_LEN + sizeof(HASH);
    byte key[keyLen] = {};
    HASH txHash = {};

    getOriginalTxHash(txHash);
    _constructKey(CALLBACK_KEY, CALLBACK_KEY_LEN, txHash, sizeof(HASH), key);
    storageLoad(key, keyLen, arg);
}

void _storeCallbackArg(const HASH nameHash)
{
    const int keyLen = CALLBACK_KEY_LEN + sizeof(HASH);
    byte key[keyLen] = {};
    HASH txHash = {};

    getOriginalTxHash(txHash);
    _constructKey(CALLBACK_KEY, CALLBACK_KEY_LEN, txHash, sizeof(HASH), key);
    storageStore(key, keyLen, nameHash, sizeof(HASH));
}

void _clearCallbackArg()
{
    const int keyLen = CALLBACK_KEY_LEN + sizeof(HASH);
    byte key[keyLen] = {};
    HASH txHash = {};

    getOriginalTxHash(txHash);
    _constructKey(CALLBACK_KEY, CALLBACK_KEY_LEN, txHash, sizeof(HASH), key);
    storageStore(key, keyLen, NULL, 0);
}

void _resolveFromHash(const HASH nameHash, ADDRESS result)
{
    Value value = {};

    if (_shardId(nameHash) == _getOwnShardId())
    {
        _loadValue(nameHash, &value);
        if (value.state == Commited)
        {
            _copy(result, value.address, sizeof(ADDRESS));
        }
    }
}

void _constructKey(const byte *prefix, int prefixLen, const byte *arg, int argLen, byte *key)
{
    _copy(key, prefix, prefixLen);
    _copyRange(key, arg, prefixLen, 0, argLen);
}

int _constructAsyncCallData(const byte *funcName, int funcLen,
                            const byte *arg, int argLen, byte *data)
{
    int i;
    int dataIndex = 0;
    byte hexEncodedData[1000] = {};
    const byte argDelimiter = '@';

    _copy(data, funcName, funcLen);
    dataIndex += funcLen;

    _copyRange(data, &argDelimiter, dataIndex, 0, 1);
    dataIndex++;

    _hexEncode(arg, argLen, hexEncodedData);
    _copyRange(data, hexEncodedData, dataIndex, 0, 2 * argLen);
    dataIndex += 2 * argLen;

    return dataIndex;
}

byte _halfByteToHexDigit(byte num)
{
    if (num < 10)
    {
        return '0' + num;
    }
    else
    {
        return 'a' + num - 0xa;
    }
}

void _hexEncode(const byte *data, int dataLen, byte *result)
{
    int i;
    for (i = 0; i < dataLen; i++)
    {
        result[i * 2] = _halfByteToHexDigit(data[i] >> 4);
        result[i * 2 + 1] = _halfByteToHexDigit(data[i] & 0x0f);
    }
}

// callback method
// first arg: return code
// second arg: data passed by finish() in other contract OR error message
void callBack()
{
    HASH nameHash = {};
    Value value = {};
    int numArgs = getNumArguments();
    int result = numArgs ? int64getArgument(0) : 0;

    _loadCallbackArg(nameHash);
    _loadValue(nameHash, &value);
    if (result == 0 && value.state == Pending)
    {
        value.state = Commited;
    }
    else
    {
        value.state = None;
        _copy(value.address, ZERO_32_BYTE_ARRAY, sizeof(ADDRESS));
    }

    _storeValue(nameHash, &value);

    _clearCallbackArg();
}

// fake memcpy
void *memcpy(void *dest, const void *src, unsigned long n);
void *memcpy(void *dest, const void *src, unsigned long n)
{
    char *csrc = (char *)src;
    char *cdest = (char *)dest;

    for (int i = 0; i < n; i++)
    {
        cdest[i] = csrc[i];
    }

    return dest;
}

// fake memset
void *memset(void *dest, int c, unsigned long n);
void *memset(void *dest, int c, unsigned long n)
{
    int i;
    char *cdest = (char *)dest;
    for (i = 0; i < n; i++)
    {
        cdest[i] = c;
    }
    return dest;
}
