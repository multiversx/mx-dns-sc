#include "elrond/context.h"
#include "elrond/util.h"

#define NULL 0
#define MIN_NAME_LENGTH 10
#define true 1
#define false 0

typedef int bool;
typedef byte ADDRESS[32];
typedef byte HASH[32];

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

byte* _validateName(byte *name, int len);
void _hashName(byte *name, int nameLen, HASH result);
int _getOwnShardId();
void _copy(byte *dest, byte *source, int len);
void _copyRange(byte *dest, byte *src, int destStart, int srcStart, int len);
bool _equal(byte *op1, byte *op2, int len);
void _loadValue(HASH key, Value *value);
void _storeValue(HASH key, Value *value);
void _resolveFromHash(HASH nameHash, ADDRESS result);
int _constructAsyncCallData(byte *funcName, int funcLen, byte **args, int *argsLen, int nrArgs, byte *data);
byte _halfByteToHexDigit(byte num);
void _hexEncode(byte *data, int dataLen, byte *result);

ADDRESS ZERO_32_BYTE_ARRAY = { 0 };

byte SET_USER_NAME_FUNCTION[] = "SetUserName";
const int SET_USER_NAME_LEN = 11;
byte CLAIM_MSG[] = "dns claim";
const int CLAIM_MSG_LEN = 9;
byte OK_RETURN_MSG[] = "ok";
const int OK_RETURN_MSG_LEN = 2;

ERROR_MSG(ERR_NAME_TOO_SHORT, "name is too short");
ERROR_MSG(ERR_CHARACTER_NOT_ALLOWED, "character not allowed");
ERROR_MSG(ERR_WRONG_FEE, "should pay exactly the registration cost");
ERROR_MSG(ERR_DIFFERENT_SHARD, "name belongs to another shard");
ERROR_MSG(ERR_NAME_ALREADY_TAKEN, "name already taken");
ERROR_MSG(ERR_CLAIM, "only owner can claim");

STORAGE_KEY(REGISTRATION_COST);

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

    byte paymentAsBytes[100] = {};
    int payLen;
    bigInt payment = bigIntNew(0);
    bigInt registrationCost = bigIntNew(0);

    byte name[100] = {};
    int nameLen;
    HASH nameHash = {};
    byte *errMsg;

    Value value = {};
    ADDRESS callerAddress = {};

    HASH txHash = { };
    byte callValueAsync[32] = { 0 };
    byte dataAsync[200] = { };
    int dataLen;
    
    payLen = getCallValue(paymentAsBytes);
    bigIntSetUnsignedBytes(payment, paymentAsBytes, payLen);
    bigIntStorageLoadUnsigned(REGISTRATION_COST_KEY, REGISTRATION_COST_KEY_LEN, 
        registrationCost);

    if (bigIntCmp(payment, registrationCost) != 0)
    {
        SIGNAL_ERROR(ERR_WRONG_FEE);
    }

    nameLen = getArgument(0, name);
    errMsg = _validateName(name, nameLen);
    if (errMsg != NULL)
    {
        signalError(errMsg, sizeof(errMsg) - 1);
    }

    _hashName(name, nameLen, nameHash);
    if (getShardOfAddress(nameHash) != _getOwnShardId())
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

    getOriginalTxHash(txHash);
   // store "fake" callback arg in storage and retrieve in callback
    storageStore(txHash, sizeof(HASH), nameHash, sizeof(HASH));

    dataLen = _constructAsyncCallData(SET_USER_NAME_FUNCTION, SET_USER_NAME_LEN, 
        (byte**)&name, &nameLen, 1, dataAsync);
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

    finish(OK_RETURN_MSG, OK_RETURN_MSG_LEN);
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

    int shardId = _getOwnShardId();

    int64finish(shardId);
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
    int shardId;

    len = getArgument(0, name);
    _hashName(name, len, hash);
    shardId = getShardOfAddress(hash);
    
    int64finish(shardId);
}

// Args:
// byte *name
void validateNameView()
{
    CHECK_NOT_PAYABLE();
    CHECK_NUM_ARGS(1);

    int len;
    byte name[100] = {};
    byte *errMsg; 
    
    len = getArgument(0, name);
    errMsg = _validateName(name, len);

    if (errMsg != NULL)
    {
        signalError(errMsg, sizeof(errMsg) - 1);
    }

    finish(OK_RETURN_MSG, OK_RETURN_MSG_LEN);
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
    ADDRESS addr = { 0 };

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
    ADDRESS addr = { 0 };

    getArgument(0, nameHash);
    _resolveFromHash(nameHash, addr);

    finish(addr, sizeof(ADDRESS));
}

// private helpers

int _checkNameChar(byte ch)
{
    return (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'z');
}

byte* _validateName(byte *name, int len)
{
    int i;

    if (len < MIN_NAME_LENGTH)
    {
        return ERR_NAME_TOO_SHORT;
    }

    for (i = 0; i < len; i++)
    {
        if (_checkNameChar(name[i]))
        {
            return ERR_CHARACTER_NOT_ALLOWED;
        }
    }

    return NULL;
}

void _hashName(byte *name, int nameLen, HASH result)
{
    keccak256(name, nameLen, result);
}

int _getOwnShardId()
{
    ADDRESS scAddr = {};
    getSCAddress(scAddr);

    return getShardOfAddress(scAddr);
}

void _copy(byte *dest, byte *src, int len)
{
    int i;
    for (i = 0; i < len; i++)
    {
        dest[i] = src[i];
    }
}

void _copyRange(byte *dest, byte *src, int destStart, int srcStart, int len)
{
    int i;
    for (int i = 0; i < len; i++)
    {
        dest[destStart + i] = src[srcStart + i];
    }
}

bool _equal(byte *op1, byte *op2, int len)
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

void _loadValue(HASH key, Value *value)
{
    storageLoad(key, sizeof(HASH), (byte*)value);
}

void _storeValue(HASH key, Value *value)
{
    storageStore(key, sizeof(HASH), (byte*)value, sizeof(Value));
}

void _resolveFromHash(HASH nameHash, ADDRESS result)
{
    Value value = {};

    if (getShardOfAddress(nameHash) == _getOwnShardId())
    {
        _loadValue(nameHash, &value);
        if (value.state == Commited)
        {
            _copy(result, value.address, sizeof(ADDRESS));
        }
    }
}

int _constructAsyncCallData(byte *funcName, int funcLen, byte **args, int *argsLen, int nrArgs, byte *data)
{
    int i;
    int dataIndex = 0;
    byte hexEncodedData[1000] = { };
    byte argDelimiter[1] = "@";

    _copy(data, funcName, funcLen);
    dataIndex += funcLen;

    for (i = 0; i < nrArgs; i++)
    {
        _copyRange(data, argDelimiter, dataIndex, 0, 1);
        dataIndex++;

        _hexEncode(args[i], argsLen[i], hexEncodedData);
        _copyRange(data, hexEncodedData, dataIndex, 0, argsLen[i]);
        dataIndex += 2 * argsLen[i];
    }

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

void _hexEncode(byte *data, int dataLen, byte *result)
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
    // return code is i32
    byte resultAsBytes[4] = {};
    int result;
    HASH txHash = {};
    HASH nameHash = {};
    Value value = {};

    getArgument(0, resultAsBytes);
    getOriginalTxHash(txHash);
    storageLoad(txHash, sizeof(HASH), nameHash);
    _loadValue(nameHash, &value);

    // return code fits in one byte, others will be 0
    result = (int)resultAsBytes[3];

    if (result == 0)
    {
        if (value.state == Pending)
        {
            value.state = Commited;
        }
        else
        {
            value.state = None;
            _copy(value.address, ZERO_32_BYTE_ARRAY, sizeof(ADDRESS));
        }
    }
    else
    {
        value.state = None;
            _copy(value.address, ZERO_32_BYTE_ARRAY, sizeof(ADDRESS));
    }
    
    _storeValue(nameHash, &value);

    // clear callback stored value
    storageStore(txHash, sizeof(HASH), NULL, 0);
}
