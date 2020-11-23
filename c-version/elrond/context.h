#ifndef _CONTEXT_H_
#define _CONTEXT_H_

#include "types.h"

void getSCAddress(byte *address);
void getOwnerAddress(byte *address);
int getShardOfAddress(const byte *address);
int isSmartContract(const byte *address);

// Call-related functions
void getOriginalTxHash(byte *txHash);
void getCaller(byte *callerAddress);
int getFunction(byte *function);
int getCallValue(byte *result);
long long getGasLeft();
void finish(const byte *data, int length);
void int64finish(long long value);
void writeLog(byte *pointer, int length, byte *topicPtr, int numTopics);
void asyncCall(const byte *destination, const byte *value, const byte *data, int length);
void signalError(byte *message, int length);

int executeOnSameContext(long long gas, byte *address, byte *value, byte *function, int functionLength, int numArguments, byte *argumentsLengths, byte *arguments);
int executeOnDestContext(long long gas, byte *address, byte *value, byte *function, int functionLength, int numArguments, byte *argumentsLengths, byte *arguments);
int createContract(long long gas, byte *value, byte *code, int codeSize, byte *newAddress, int numInitArgs, byte *initArgLengths, byte *initArgs);

// Blockchain-related functions
long long getBlockTimestamp();
int getBlockHash(long long nonce, byte *hash);

// Argument-related functions
int getNumArguments();
int getArgument(int argumentIndex, byte *argument);
long long int64getArgument(int argumentIndex);
int getArgumentLength(int argumentIndex);

// Account-related functions
void getExternalBalance(const byte *address, byte *balance);
int transferValue(const byte *destination, const byte *value, const byte *data, int length);

// Storage-related functions
int storageLoadLength(const byte *key, int keyLength);
int storageStore(const byte *key, int keyLength, const byte *data, int dataLength);
int storageLoad(const byte *key, int keyLength, byte *data);
int int64storageStore(const byte *key, int keyLength, long long value);
long long int64storageLoad(const byte *key, int keyLength);

// Timelocks-related functions
int setStorageLock(byte *key, int keyLen, long long timeLock);
long long getStorageLock(byte *key, int keyLen);
int isStorageLocked(byte *key, int keyLen);
int clearStorageLock(byte *key, int keyLen);

// Crypto-related functions
int keccak256(const byte *key, int keyLength, byte *result);
int sha256(const byte *key, int keyLength, byte *result);

// Debugging
void myDebug(const byte *data, int length);

#endif
