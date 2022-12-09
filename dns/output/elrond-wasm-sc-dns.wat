(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32 i64)))
  (type (;11;) (func (param i32 i32) (result i64)))
  (type (;12;) (func (param i32 i32 i32 i32 i32)))
  (type (;13;) (func (param i32 i64 i32)))
  (type (;14;) (func (param i64)))
  (type (;15;) (func (param i32 i32 i64 i32 i32) (result i32)))
  (type (;16;) (func (param i32) (result i64)))
  (import "env" "signalError" (func (;0;) (type 0)))
  (import "env" "mBufferNew" (func (;1;) (type 4)))
  (import "env" "mBufferAppend" (func (;2;) (type 5)))
  (import "env" "managedCaller" (func (;3;) (type 3)))
  (import "env" "managedSCAddress" (func (;4;) (type 3)))
  (import "env" "managedOwnerAddress" (func (;5;) (type 3)))
  (import "env" "mBufferEq" (func (;6;) (type 5)))
  (import "env" "mBufferAppendBytes" (func (;7;) (type 8)))
  (import "env" "managedSignalError" (func (;8;) (type 3)))
  (import "env" "mBufferGetLength" (func (;9;) (type 2)))
  (import "env" "smallIntGetUnsignedArgument" (func (;10;) (type 16)))
  (import "env" "bigIntGetUnsignedArgument" (func (;11;) (type 0)))
  (import "env" "getNumArguments" (func (;12;) (type 4)))
  (import "env" "mBufferFinish" (func (;13;) (type 2)))
  (import "env" "managedGetOriginalTxHash" (func (;14;) (type 3)))
  (import "env" "mBufferGetByteSlice" (func (;15;) (type 9)))
  (import "env" "mBufferSetBytes" (func (;16;) (type 8)))
  (import "env" "mBufferFromBigIntUnsigned" (func (;17;) (type 5)))
  (import "env" "bigIntSetInt64" (func (;18;) (type 10)))
  (import "env" "mBufferCopyByteSlice" (func (;19;) (type 9)))
  (import "env" "mBufferStorageLoad" (func (;20;) (type 5)))
  (import "env" "mBufferStorageStore" (func (;21;) (type 5)))
  (import "env" "mBufferGetArgument" (func (;22;) (type 5)))
  (import "env" "managedAsyncCall" (func (;23;) (type 7)))
  (import "env" "managedKeccak256" (func (;24;) (type 5)))
  (import "env" "mBufferToBigIntUnsigned" (func (;25;) (type 5)))
  (import "env" "checkNoPayment" (func (;26;) (type 1)))
  (import "env" "getNumESDTTransfers" (func (;27;) (type 4)))
  (import "env" "bigIntGetCallValue" (func (;28;) (type 3)))
  (import "env" "bigIntCmp" (func (;29;) (type 5)))
  (import "env" "bigIntGetExternalBalance" (func (;30;) (type 0)))
  (import "env" "managedTransferValueExecute" (func (;31;) (type 15)))
  (import "env" "bigIntFinishUnsigned" (func (;32;) (type 3)))
  (import "env" "smallIntFinishUnsigned" (func (;33;) (type 14)))
  (import "env" "finish" (func (;34;) (type 0)))
  (import "env" "mBufferGetBytes" (func (;35;) (type 5)))
  (func (;36;) (type 12) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.ge_u
      if  ;; label = @2
        local.get 2
        local.get 4
        i32.le_u
        br_if 1 (;@1;)
        call 37
        unreachable
      end
      call 37
      unreachable
    end
    local.get 0
    local.get 2
    local.get 1
    i32.sub
    i32.store offset=4
    local.get 0
    local.get 1
    local.get 3
    i32.add
    i32.store)
  (func (;37;) (type 1)
    call 104
    unreachable)
  (func (;38;) (type 2) (param i32) (result i32)
    (local i32)
    call 1
    local.tee 1
    local.get 0
    call 2
    drop
    local.get 1)
  (func (;39;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 0
    unreachable)
  (func (;40;) (type 4) (result i32)
    (local i32)
    call 41
    local.tee 0
    call 3
    local.get 0)
  (func (;41;) (type 4) (result i32)
    (local i32)
    i32.const 1049352
    i32.const 1049352
    i32.load
    i32.const -1
    i32.add
    local.tee 0
    i32.store
    local.get 0)
  (func (;42;) (type 4) (result i32)
    (local i32)
    call 41
    local.tee 0
    call 4
    local.get 0)
  (func (;43;) (type 4) (result i32)
    (local i32)
    call 41
    local.tee 0
    call 5
    local.get 0)
  (func (;44;) (type 1)
    call 43
    call 40
    call 6
    i32.const 1
    i32.ge_s
    if  ;; label = @1
      return
    end
    i32.const 1049300
    i32.const 36
    call 0
    unreachable)
  (func (;45;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    i32.const 1048628
    i32.const 23
    call 46
    local.tee 4
    local.get 0
    local.get 1
    call 7
    drop
    local.get 4
    i32.const 1048651
    i32.const 3
    call 7
    drop
    local.get 4
    local.get 2
    local.get 3
    call 7
    drop
    local.get 4
    call 8
    unreachable)
  (func (;46;) (type 5) (param i32 i32) (result i32)
    (local i32)
    call 41
    local.tee 2
    local.get 0
    local.get 1
    call 16
    drop
    local.get 2)
  (func (;47;) (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=8
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      i32.load
      local.get 3
      i32.const 2
      i32.shl
      local.get 1
      i32.const 12
      i32.add
      i32.const 4
      call 48
      i32.eqz
      if  ;; label = @2
        local.get 1
        i32.load offset=12
        local.set 2
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=8
        local.get 2
        i32.const 8
        i32.shl
        i32.const 16711680
        i32.and
        local.get 2
        i32.const 24
        i32.shl
        i32.or
        local.get 2
        i32.const 8
        i32.shr_u
        i32.const 65280
        i32.and
        local.get 2
        i32.const 24
        i32.shr_u
        i32.or
        i32.or
        call 38
        call 38
        local.tee 0
        call 9
        i32.const 32
        i32.ne
        br_if 1 (;@1;)
        local.get 1
        i32.const 16
        i32.add
        global.set 0
        local.get 0
        return
      end
      i32.const 1049065
      i32.const 12
      i32.const 1048691
      i32.const 17
      call 45
      unreachable
    end
    i32.const 1049065
    i32.const 12
    i32.const 1048888
    i32.const 16
    call 45
    unreachable)
  (func (;48;) (type 9) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 3
    local.get 2
    call 15
    i32.const 0
    i32.ne)
  (func (;49;) (type 0) (param i32 i32)
    (local i32 i32 i64)
    local.get 1
    call 50
    call 10
    local.tee 4
    i64.const 4294967296
    i64.lt_u
    if  ;; label = @1
      block (result i32)  ;; label = @2
        i32.const 0
        local.get 4
        i32.wrap_i64
        local.tee 2
        i32.eqz
        local.get 2
        i32.const 28523
        i32.eq
        i32.or
        br_if 0 (;@2;)
        drop
        local.get 1
        i32.load
        i32.const 1059364
        i32.load
        i32.ge_s
        if  ;; label = @3
          call 51
          local.set 3
          i32.const 1
          br 1 (;@2;)
        end
        local.get 1
        call 50
        call 52
        local.set 3
        i32.const 1
      end
      local.set 1
      local.get 0
      local.get 3
      i32.store offset=8
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      return
    end
    i32.const 1049059
    i32.const 6
    i32.const 1048589
    i32.const 14
    call 45
    unreachable)
  (func (;50;) (type 2) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 1
    i32.const 1059364
    i32.load
    i32.ge_s
    if  ;; label = @1
      i32.const 1049059
      i32.const 6
      i32.const 1048691
      i32.const 17
      call 45
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 1
    i32.add
    i32.store
    local.get 1)
  (func (;51;) (type 4) (result i32)
    (local i32)
    call 41
    local.tee 0
    i32.const 1049300
    i32.const 0
    call 16
    drop
    local.get 0)
  (func (;52;) (type 2) (param i32) (result i32)
    local.get 0
    call 41
    local.tee 0
    call 22
    drop
    local.get 0)
  (func (;53;) (type 4) (result i32)
    (local i64)
    block (result i32)  ;; label = @1
      block  ;; label = @2
        i32.const 1
        call 10
        local.tee 0
        i64.const 1
        i64.le_u
        if  ;; label = @3
          i32.const 0
          local.get 0
          i32.wrap_i64
          i32.const 1
          i32.sub
          br_if 2 (;@1;)
          drop
          br 1 (;@2;)
        end
        i32.const 1049276
        i32.const 5
        i32.const 1048848
        i32.const 18
        call 45
        unreachable
      end
      i32.const 1
    end)
  (func (;54;) (type 4) (result i32)
    (local i32)
    i32.const 0
    call 52
    local.tee 0
    call 9
    i32.const 32
    i32.ne
    if  ;; label = @1
      i32.const 1049077
      i32.const 9
      i32.const 1048888
      i32.const 16
      call 45
      unreachable
    end
    local.get 0)
  (func (;55;) (type 3) (param i32)
    call 12
    local.get 0
    i32.eq
    if  ;; label = @1
      return
    end
    i32.const 1048726
    i32.const 25
    call 0
    unreachable)
  (func (;56;) (type 1)
    i32.const 1059364
    call 12
    i32.store)
  (func (;57;) (type 0) (param i32 i32)
    local.get 0
    i32.eqz
    if  ;; label = @1
      local.get 1
      call 13
      drop
    end)
  (func (;58;) (type 3) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=16
    local.tee 2
    if  ;; label = @1
      call 59
      local.get 1
      i32.const 8
      i32.add
      call 60
      local.get 1
      local.get 1
      i32.load8_u offset=12
      i32.store8 offset=20
      local.get 1
      local.get 1
      i32.load offset=8
      i32.store offset=16
      local.get 0
      i32.const 20
      i32.add
      i32.load
      local.tee 3
      local.get 1
      i32.const 16
      i32.add
      call 61
      local.get 1
      i32.const 16
      i32.add
      local.get 2
      local.get 3
      call 62
      local.get 0
      i32.const 24
      i32.add
      i32.load
      local.tee 4
      call 63
      local.get 1
      i32.const 16
      i32.add
      call 61
      local.get 4
      call 9
      local.set 6
      i32.const 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        i32.const 4
        i32.add
        local.tee 3
        local.get 6
        i32.le_u
        if  ;; label = @3
          local.get 1
          i32.const 0
          i32.store offset=28
          local.get 4
          local.get 2
          local.get 1
          i32.const 28
          i32.add
          i32.const 4
          call 48
          drop
          local.get 1
          local.get 1
          i32.load offset=28
          local.tee 2
          i32.const 24
          i32.shl
          local.get 2
          i32.const 8
          i32.shl
          i32.const 16711680
          i32.and
          i32.or
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 65280
          i32.and
          local.get 2
          i32.const 24
          i32.shr_u
          i32.or
          i32.or
          local.tee 7
          call 9
          local.tee 2
          i32.const 24
          i32.shl
          local.get 2
          i32.const 8
          i32.shl
          i32.const 16711680
          i32.and
          i32.or
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 65280
          i32.and
          local.get 2
          i32.const 24
          i32.shr_u
          i32.or
          i32.or
          i32.store offset=28
          local.get 1
          i32.const 16
          i32.add
          local.get 1
          i32.const 28
          i32.add
          i32.const 4
          call 62
          local.get 1
          i32.const 16
          i32.add
          local.get 7
          call 64
          local.get 3
          local.set 2
          br 1 (;@2;)
        end
      end
      local.get 1
      i32.load offset=16
      local.get 1
      i32.load8_u offset=20
      call 65
    end
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 23
    unreachable)
  (func (;59;) (type 4) (result i32)
    (local i32 i32)
    call 41
    local.tee 0
    call 14
    i32.const 1048798
    i32.const 10
    call 46
    local.tee 1
    local.get 0
    call 2
    drop
    local.get 1)
  (func (;60;) (type 3) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    call 89
    local.get 1
    i32.load offset=8
    local.set 2
    local.get 0
    local.get 1
    i32.load8_u offset=12
    i32.const 1
    i32.and
    i32.store8 offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;61;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    local.get 0
    i32.const 24
    i32.shl
    i32.or
    local.get 0
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 0
    i32.const 24
    i32.shr_u
    i32.or
    i32.or
    i32.store offset=12
    local.get 1
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 62
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;62;) (type 6) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        if  ;; label = @3
          i32.const 10000
          i32.const 1059356
          i32.load
          local.tee 4
          i32.sub
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          local.get 2
          local.get 4
          i32.add
          local.tee 0
          call 87
          local.get 3
          i32.load offset=8
          local.get 3
          i32.load offset=12
          local.get 1
          local.get 2
          call 81
          i32.const 1059356
          local.get 0
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.get 1
        local.get 2
        call 7
        drop
        br 1 (;@1;)
      end
      local.get 0
      call 88
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call 7
      drop
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;63;) (type 2) (param i32) (result i32)
    local.get 0
    call 9
    i32.const 2
    i32.shr_u)
  (func (;64;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u offset=4
    local.set 3
    local.get 0
    i32.const 0
    i32.store8 offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.and
          local.tee 3
          if  ;; label = @4
            local.get 1
            call 9
            local.set 4
            i32.const 10000
            i32.const 1059356
            i32.load
            local.tee 5
            i32.sub
            local.get 4
            i32.lt_u
            br_if 2 (;@2;)
            local.get 2
            i32.const 8
            i32.add
            local.get 5
            local.get 4
            local.get 5
            i32.add
            local.tee 4
            call 87
            local.get 1
            i32.const 0
            local.get 2
            i32.load offset=8
            local.get 2
            i32.load offset=12
            call 48
            drop
            i32.const 1059356
            local.get 4
            i32.store
            br 1 (;@3;)
          end
          local.get 0
          i32.load
          local.get 1
          call 79
        end
        local.get 0
        local.get 3
        i32.store8 offset=4
        br 1 (;@1;)
      end
      local.get 0
      call 88
      local.get 0
      i32.load
      local.get 1
      call 79
      local.get 0
      i32.load8_u offset=4
      local.get 0
      local.get 3
      i32.store8 offset=4
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1059356
      i32.const 0
      i32.store
      i32.const 1059360
      i32.const 0
      i32.store8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;65;) (type 6) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call 90
    call 21
    drop)
  (func (;66;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load offset=28
    local.set 11
    local.get 1
    i32.load offset=24
    local.set 8
    local.get 1
    i32.load offset=20
    local.set 5
    local.get 1
    i32.load offset=12
    local.set 6
    local.get 1
    i32.load offset=8
    local.set 4
    local.get 1
    i64.load
    local.set 15
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 7
            call 67
            br_table 3 (;@1;) 1 (;@3;) 0 (;@4;)
          end
          call 51
          local.set 9
          call 51
          local.set 3
          call 51
          drop
          local.get 3
          local.get 4
          call 38
          call 68
          local.get 7
          call 67
          local.set 4
          call 51
          local.tee 6
          local.get 4
          i64.extend_i32_u
          call 69
          local.get 3
          local.get 6
          call 68
          local.get 7
          call 9
          local.set 10
          local.get 2
          i32.const 16
          i32.add
          local.set 12
          i32.const 0
          local.set 4
          loop  ;; label = @4
            local.get 4
            i32.const 16
            i32.add
            local.tee 6
            local.get 10
            i32.gt_u
            if  ;; label = @5
              local.get 5
              call 70
              i32.eqz
              if  ;; label = @6
                call 51
                drop
                local.get 3
                local.get 5
                call 38
                call 68
              end
              call 42
              local.set 4
              call 71
              local.set 6
              local.get 9
              local.set 7
              i32.const 1048751
              i32.const 20
              call 46
              br 3 (;@2;)
            else
              local.get 12
              i64.const 0
              i64.store
              local.get 2
              i64.const 0
              i64.store offset=8
              local.get 7
              local.get 4
              local.get 2
              i32.const 8
              i32.add
              i32.const 16
              call 48
              drop
              local.get 2
              i32.const 0
              i32.store offset=28
              local.get 2
              i32.const 8
              i32.add
              local.get 2
              i32.const 28
              i32.add
              call 72
              local.set 4
              local.get 2
              i32.const 8
              i32.add
              local.get 2
              i32.const 28
              i32.add
              call 73
              local.set 14
              local.get 2
              i32.const 8
              i32.add
              local.get 2
              i32.const 28
              i32.add
              call 72
              local.set 13
              call 51
              drop
              local.get 3
              local.get 4
              call 38
              call 68
              local.get 3
              local.get 14
              call 74
              call 51
              drop
              local.get 3
              local.get 13
              call 75
              call 68
              local.get 6
              local.set 4
              br 1 (;@4;)
            end
            unreachable
          end
          unreachable
        end
        local.get 2
        i32.const 16
        i32.add
        i64.const 0
        i64.store
        local.get 2
        i64.const 0
        i64.store offset=8
        local.get 7
        i32.const 0
        local.get 2
        i32.const 8
        i32.add
        i32.const 16
        call 48
        local.get 2
        i32.const 0
        i32.store offset=28
        local.get 2
        i32.const 8
        i32.add
        local.get 2
        i32.const 28
        i32.add
        call 72
        local.set 10
        local.get 2
        i32.const 8
        i32.add
        local.get 2
        i32.const 28
        i32.add
        call 73
        local.set 14
        local.get 2
        i32.const 8
        i32.add
        local.get 2
        i32.const 28
        i32.add
        call 72
        local.set 9
        br_if 1 (;@1;)
        call 51
        local.set 7
        call 51
        local.tee 3
        local.get 10
        call 76
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 14
              i64.eqz
              if  ;; label = @6
                local.get 3
                local.get 9
                call 77
                local.get 5
                call 70
                i32.eqz
                br_if 1 (;@5;)
                br 3 (;@3;)
              end
              local.get 3
              local.get 14
              call 74
              local.get 3
              local.get 9
              call 77
              call 51
              drop
              local.get 3
              local.get 4
              call 38
              call 68
              local.get 5
              call 70
              br_if 1 (;@4;)
              local.get 3
              local.get 5
              call 76
              br 1 (;@4;)
            end
            local.get 3
            local.get 5
            call 76
            br 1 (;@3;)
          end
          call 42
          local.set 4
          call 71
          local.set 6
          i32.const 1048771
          i32.const 15
          call 46
          br 1 (;@2;)
        end
        call 71
        local.set 6
        i32.const 1048786
        i32.const 12
        call 46
      end
      local.set 5
      local.get 3
      local.get 8
      call 2
      drop
      local.get 3
      local.set 8
    end
    local.get 1
    local.get 11
    i32.store offset=28
    local.get 1
    local.get 8
    i32.store offset=24
    local.get 1
    local.get 5
    i32.store offset=20
    local.get 1
    local.get 7
    i32.store offset=16
    local.get 1
    local.get 6
    i32.store offset=12
    local.get 1
    local.get 4
    i32.store offset=8
    local.get 1
    local.get 15
    i64.store
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    local.get 8
    i32.store offset=12
    local.get 0
    local.get 5
    i32.store offset=8
    local.get 0
    local.get 6
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;67;) (type 2) (param i32) (result i32)
    local.get 0
    call 9
    i32.const 4
    i32.shr_u)
  (func (;68;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    local.get 1
    i32.const 24
    i32.shl
    i32.or
    local.get 1
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 1
    i32.const 24
    i32.shr_u
    i32.or
    i32.or
    i32.store offset=12
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 7
    drop
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;69;) (type 10) (param i32 i64)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i64.const 0
    i64.store offset=8
    local.get 2
    local.get 1
    local.get 2
    i32.const 8
    i32.add
    call 93
    local.get 0
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 16
    drop
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;70;) (type 2) (param i32) (result i32)
    local.get 0
    call 9
    i32.eqz)
  (func (;71;) (type 4) (result i32)
    (local i32)
    call 41
    local.tee 0
    i64.const 0
    call 18
    local.get 0)
  (func (;72;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    local.get 2
    local.get 0
    local.get 1
    i32.load
    local.tee 0
    local.get 0
    i32.const 4
    i32.add
    local.tee 0
    call 80
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 81
    local.get 1
    local.get 0
    i32.store
    local.get 2
    i32.load offset=12
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    local.get 0
    i32.const 24
    i32.shl
    i32.or
    local.get 0
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 0
    i32.const 24
    i32.shr_u
    i32.or
    i32.or)
  (func (;73;) (type 11) (param i32 i32) (result i64)
    (local i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i64.const 0
    i64.store offset=8
    local.get 2
    local.get 0
    local.get 1
    i32.load
    local.tee 0
    local.get 0
    i32.const 8
    i32.add
    local.tee 0
    call 80
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 81
    local.get 1
    local.get 0
    i32.store
    local.get 2
    i64.load offset=8
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 3
    i64.const 40
    i64.shl
    i64.const 71776119061217280
    i64.and
    local.get 3
    i64.const 56
    i64.shl
    i64.or
    local.get 3
    i64.const 24
    i64.shl
    i64.const 280375465082880
    i64.and
    local.get 3
    i64.const 8
    i64.shl
    i64.const 1095216660480
    i64.and
    i64.or
    i64.or
    local.get 3
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 3
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 3
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 3
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or)
  (func (;74;) (type 10) (param i32 i64)
    (local i32)
    call 51
    local.tee 2
    local.get 1
    call 69
    local.get 0
    local.get 2
    call 68)
  (func (;75;) (type 2) (param i32) (result i32)
    (local i32)
    call 41
    local.tee 1
    local.get 0
    call 17
    drop
    local.get 1)
  (func (;76;) (type 0) (param i32 i32)
    call 51
    drop
    local.get 0
    local.get 1
    call 38
    call 68)
  (func (;77;) (type 0) (param i32 i32)
    call 51
    drop
    local.get 0
    local.get 1
    call 75
    call 68)
  (func (;78;) (type 0) (param i32 i32)
    (local i32)
    local.get 1
    call 9
    local.set 2
    local.get 0
    i32.const 16
    i32.add
    i32.const 0
    i32.store8
    local.get 0
    i32.const 12
    i32.add
    local.get 2
    i32.store
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store)
  (func (;79;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 2
    drop)
  (func (;80;) (type 7) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 3
      local.get 2
      i32.ge_u
      if  ;; label = @2
        local.get 3
        i32.const 16
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        i32.sub
        i32.store offset=4
        local.get 0
        local.get 1
        local.get 2
        i32.add
        i32.store
        return
      end
      call 37
      unreachable
    end
    call 37
    unreachable)
  (func (;81;) (type 7) (param i32 i32 i32 i32)
    local.get 1
    local.get 3
    i32.eq
    if  ;; label = @1
      local.get 0
      local.get 2
      local.get 1
      call 133
      return
    end
    call 104
    unreachable)
  (func (;82;) (type 0) (param i32 i32)
    local.get 0
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 24
    i32.add
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 16
    i32.add
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 8
    i32.add
    i64.const 0
    i64.store align=1
    local.get 1
    i32.const 0
    local.get 0
    i32.const 32
    call 48
    drop)
  (func (;83;) (type 7) (param i32 i32 i32 i32)
    local.get 1
    local.get 2
    local.get 3
    call 1
    local.tee 1
    call 19
    local.set 2
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.eqz
    i32.store)
  (func (;84;) (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=8
        i32.eqz
        if  ;; label = @3
          local.get 0
          i32.load
          local.tee 6
          call 9
          local.set 5
          i32.const 1059360
          i32.load8_u
          local.get 5
          i32.const 10000
          i32.gt_u
          i32.or
          br_if 1 (;@2;)
          i32.const 1059356
          local.get 5
          i32.store
          i32.const 1059360
          i32.const 1
          i32.store8
          local.get 4
          i32.const 8
          i32.add
          local.get 5
          call 85
          local.get 6
          i32.const 0
          local.get 4
          i32.load offset=8
          local.get 4
          i32.load offset=12
          call 48
          drop
          local.get 0
          i32.const 1
          i32.store8 offset=8
        end
        i32.const 1
        local.get 1
        local.get 3
        i32.add
        local.tee 0
        i32.const 1059356
        i32.load
        i32.gt_u
        br_if 1 (;@1;)
        drop
        local.get 4
        local.get 1
        local.get 0
        call 86
        local.get 2
        local.get 3
        local.get 4
        i32.load
        local.get 4
        i32.load offset=4
        call 81
        i32.const 0
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store8 offset=8
      local.get 6
      local.get 1
      local.get 2
      local.get 3
      call 48
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;85;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 1049356
    i32.const 10000
    local.get 1
    call 111
    local.get 2
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 2
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;86;) (type 6) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.const 10000
        i32.le_u
        br_if 1 (;@1;)
        call 37
        unreachable
      end
      call 37
      unreachable
    end
    local.get 0
    local.get 2
    local.get 1
    i32.sub
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 1049356
    i32.add
    i32.store)
  (func (;87;) (type 6) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    i32.const 1049356
    i32.const 10000
    call 36
    local.get 3
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;88;) (type 3) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load8_u offset=4
    local.get 0
    i32.const 0
    i32.store8 offset=4
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 1
      i32.const 8
      i32.add
      i32.const 0
      i32.const 1059356
      i32.load
      call 86
      local.get 0
      i32.load
      local.get 1
      i32.load offset=8
      local.get 1
      i32.load offset=12
      call 7
      drop
      i32.const 1059356
      i32.const 0
      i32.store
      i32.const 1059360
      i32.const 0
      i32.store8
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;89;) (type 3) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    block (result i32)  ;; label = @1
      i32.const 1059360
      i32.load8_u
      local.tee 2
      i32.eqz
      if  ;; label = @2
        i32.const 1059360
        i32.const 1
        i32.store8
        i32.const 1059356
        i32.const 0
        i32.store
        local.get 1
        i32.const 8
        i32.add
        i32.const 0
        call 85
        local.get 1
        i32.load offset=8
        local.get 1
        i32.load offset=12
        i32.const 1049300
        i32.const 0
        call 81
        call 51
        br 1 (;@1;)
      end
      i32.const 1049300
      i32.const 0
      call 46
    end
    i32.store
    local.get 0
    local.get 2
    i32.const 1
    i32.xor
    i32.store8 offset=4
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;90;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store8 offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call 88
    local.get 2
    i32.load offset=8
    local.get 2
    i32.load8_u offset=12
    if  ;; label = @1
      i32.const 1059356
      i32.const 0
      i32.store
      i32.const 1059360
      i32.const 0
      i32.store8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;91;) (type 2) (param i32) (result i32)
    local.get 0
    call 41
    local.tee 0
    call 20
    drop
    local.get 0)
  (func (;92;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i64.const 0
    i64.store offset=8
    local.get 2
    local.get 0
    i64.extend_i32_u
    i64.const 255
    i64.and
    local.get 2
    i32.const 8
    i32.add
    call 93
    local.get 1
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 46
    call 21
    drop
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;93;) (type 13) (param i32 i64 i32)
    (local i32 i32)
    local.get 2
    local.get 1
    i64.const 40
    i64.shl
    i64.const 71776119061217280
    i64.and
    local.get 1
    i64.const 56
    i64.shl
    i64.or
    local.get 1
    i64.const 24
    i64.shl
    i64.const 280375465082880
    i64.and
    local.get 1
    i64.const 8
    i64.shl
    i64.const 1095216660480
    i64.and
    i64.or
    i64.or
    local.get 1
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 1
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 1
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 1
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store align=1
    block  ;; label = @1
      local.get 1
      i64.eqz
      if  ;; label = @2
        i32.const 1049300
        local.set 4
        br 1 (;@1;)
      end
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 8
            i32.ne
            if  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              local.tee 4
              i32.load8_u
              i32.eqz
              br_if 2 (;@3;)
              local.get 3
              i32.const 9
              i32.ge_u
              br_if 1 (;@4;)
              i32.const 8
              local.get 3
              i32.sub
              local.set 3
              br 4 (;@1;)
            end
            call 104
            unreachable
          end
          call 132
          unreachable
        end
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 0 (;@2;)
      end
      unreachable
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func (;94;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store8 offset=15
    local.get 1
    local.get 2
    i32.const 15
    i32.add
    i32.const 1
    call 62
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;95;) (type 2) (param i32) (result i32)
    (local i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 0
    i32.store offset=12
    local.get 0
    i32.const 8
    i32.add
    local.get 0
    i32.load
    local.get 1
    i32.const 12
    i32.add
    i32.const 4
    call 84
    if  ;; label = @1
      i32.const 1048808
      i32.const 15
      call 99
      unreachable
    end
    local.get 0
    local.get 0
    i32.load
    i32.const 4
    i32.add
    i32.store
    local.get 1
    i32.const 12
    i32.add
    i32.const 4
    call 96
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    i32.wrap_i64)
  (func (;96;) (type 11) (param i32 i32) (result i64)
    (local i64)
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.const -1
        i32.add
        local.set 1
        local.get 0
        i64.load8_u
        local.get 2
        i64.const 8
        i64.shl
        i64.or
        local.set 2
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        br 0 (;@2;)
      end
      unreachable
    end
    local.get 2)
  (func (;97;) (type 0) (param i32 i32)
    (local i32)
    i32.const 1048866
    i32.const 22
    call 46
    local.tee 2
    local.get 0
    local.get 1
    call 7
    drop
    local.get 2
    call 8
    unreachable)
  (func (;98;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    call 95
    local.set 2
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load
    local.tee 3
    local.get 2
    call 83
    local.get 1
    i32.load offset=8
    i32.const 1
    i32.ne
    if  ;; label = @1
      i32.const 1048808
      i32.const 15
      call 99
      unreachable
    end
    local.get 1
    i32.load offset=12
    local.get 0
    local.get 2
    local.get 3
    i32.add
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;99;) (type 0) (param i32 i32)
    (local i32)
    i32.const 1048603
    i32.const 25
    call 46
    local.tee 2
    local.get 0
    local.get 1
    call 7
    drop
    local.get 2
    call 8
    unreachable)
  (func (;100;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.load
    local.set 3
    local.get 1
    i32.const 2147483646
    i32.store
    local.get 3
    i32.const 2147483646
    i32.eq
    if  ;; label = @1
      i32.const 1048823
      i32.const 25
      call 0
      unreachable
    end
    call 51
    local.set 4
    i32.const 1048904
    i32.const 11
    call 46
    local.set 5
    call 51
    local.set 1
    call 71
    local.set 6
    local.get 0
    local.get 5
    i32.store offset=20
    local.get 0
    local.get 4
    i32.store offset=16
    local.get 0
    local.get 6
    i32.store offset=12
    local.get 0
    local.get 3
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=24
    local.get 0
    i64.const -1
    i64.store
    call 51
    drop
    local.get 1
    local.get 2
    call 38
    call 68)
  (func (;101;) (type 0) (param i32 i32)
    (local i32)
    local.get 0
    call 51
    local.tee 2
    i32.store offset=8
    local.get 0
    i32.const 22
    i32.store offset=4
    local.get 0
    i32.const 1048915
    i32.store
    call 51
    drop
    local.get 2
    local.get 1
    call 38
    call 68)
  (func (;102;) (type 3) (param i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block (result i32)  ;; label = @3
          local.get 0
          call 9
          local.tee 3
          i32.const 8
          i32.lt_u
          if  ;; label = @4
            i32.const 28
            local.set 1
            i32.const 1048937
            br 1 (;@3;)
          end
          local.get 3
          i32.const 32
          i32.gt_u
          if  ;; label = @4
            i32.const 13
            local.set 1
            i32.const 1048965
            br 1 (;@3;)
          end
          i32.const 24
          local.set 1
          local.get 2
          i32.const 40
          i32.add
          i64.const 0
          i64.store
          local.get 2
          i32.const 32
          i32.add
          i64.const 0
          i64.store
          local.get 2
          i32.const 24
          i32.add
          i64.const 0
          i64.store
          local.get 2
          i64.const 0
          i64.store offset=16
          local.get 2
          i32.const 8
          i32.add
          local.get 2
          i32.const 16
          i32.add
          local.get 3
          call 103
          i32.const 1048978
          local.get 0
          i32.const 0
          local.get 2
          i32.load offset=8
          local.tee 3
          local.get 2
          i32.load offset=12
          local.tee 4
          call 48
          br_if 0 (;@3;)
          drop
          local.get 4
          local.get 0
          call 9
          i32.const -7
          i32.add
          local.tee 0
          i32.lt_u
          br_if 2 (;@1;)
          local.get 0
          local.get 3
          i32.add
          local.set 1
          local.get 4
          local.get 0
          i32.sub
          local.tee 4
          i32.const 7
          i32.eq
          if (result i32)  ;; label = @4
            local.get 1
            i32.const 1049002
            local.get 4
            call 135
            i32.eqz
          else
            i32.const 0
          end
          i32.eqz
          if  ;; label = @4
            i32.const 12
            local.set 1
            i32.const 1049009
            br 1 (;@3;)
          end
          local.get 0
          i32.const 3
          i32.lt_u
          if  ;; label = @4
            i32.const 17
            local.set 1
            i32.const 1049021
            br 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 0
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 3
            i32.load8_u
            local.set 4
            local.get 3
            i32.const 1
            i32.add
            local.tee 1
            local.set 3
            local.get 4
            i32.const -97
            i32.add
            i32.const 255
            i32.and
            i32.const 26
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.set 3
            local.get 4
            i32.const -48
            i32.add
            i32.const 255
            i32.and
            i32.const 10
            i32.lt_u
            br_if 0 (;@4;)
          end
          i32.const 21
          local.set 1
          i32.const 1049038
        end
        local.get 1
        call 39
        unreachable
      end
      local.get 2
      i32.const 48
      i32.add
      global.set 0
      return
    end
    call 104
    unreachable)
  (func (;103;) (type 6) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    i32.const 32
    local.get 2
    call 111
    local.get 3
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;104;) (type 1)
    call 132
    unreachable)
  (func (;105;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    call 82
    block (result i32)  ;; label = @1
      i32.const 1
      local.get 2
      i32.load8_u offset=47
      call 106
      i32.const 255
      i32.and
      i32.ne
      br_if 0 (;@1;)
      drop
      local.get 2
      i32.const 8
      i32.add
      local.get 1
      call 107
      local.get 2
      i32.load offset=12
      local.set 3
      local.get 2
      i32.load offset=8
      i32.const 2
      i32.ne
    end
    local.set 1
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;106;) (type 4) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call 42
    call 82
    local.get 0
    i32.load8_u offset=31
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func (;107;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1049281
    i32.const 11
    call 46
    local.tee 3
    local.get 1
    call 79
    local.get 3
    i32.const -25
    call 20
    drop
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const -25
            call 9
            i32.eqz
            if  ;; label = @5
              i32.const 0
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            i32.const 8
            i32.add
            local.get 3
            call 91
            call 78
            local.get 2
            i32.const 0
            i32.store8 offset=31
            local.get 2
            i32.const 16
            i32.add
            local.get 2
            i32.load offset=8
            local.get 2
            i32.const 31
            i32.add
            i32.const 1
            call 84
            br_if 1 (;@3;)
            local.get 2
            i32.load offset=8
            local.tee 5
            i32.const 1
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load8_u offset=31
                    local.tee 1
                    br_table 3 (;@5;) 2 (;@6;) 1 (;@7;) 0 (;@8;)
                  end
                  i32.const 1048576
                  i32.const 13
                  call 97
                  unreachable
                end
                i32.const 2
                local.set 1
              end
              local.get 2
              local.get 2
              i32.load offset=16
              local.get 4
              i32.const 32
              call 83
              local.get 2
              i32.load
              i32.const 1
              i32.ne
              br_if 3 (;@2;)
              local.get 2
              i32.load offset=4
              local.set 3
              local.get 5
              i32.const 33
              i32.add
              local.set 4
            end
            local.get 2
            i32.load offset=12
            local.get 4
            i32.ne
            br_if 3 (;@1;)
            local.get 2
            i32.const 24
            i32.add
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1059356
            i32.const 0
            i32.store
            i32.const 1059360
            i32.const 0
            i32.store8
          end
          local.get 0
          local.get 3
          i32.store offset=4
          local.get 0
          local.get 1
          i32.store
          local.get 2
          i32.const 32
          i32.add
          global.set 0
          return
        end
        i32.const 1048808
        i32.const 15
        call 97
        unreachable
      end
      i32.const 1048808
      i32.const 15
      call 97
      unreachable
    end
    i32.const 1048589
    i32.const 14
    call 97
    unreachable)
  (func (;108;) (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    call 82
    local.get 1
    i32.load8_u offset=31
    call 106
    i32.const 255
    i32.and
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.const 32
      i32.add
      global.set 0
      return
    end
    i32.const 1049157
    i32.const 29
    call 39
    unreachable)
  (func (;109;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1049186
    i32.const 8
    call 46
    call 110
    local.set 3
    local.get 2
    i64.const 0
    i64.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          call 91
          local.tee 3
          call 9
          local.tee 4
          i32.const 9
          i32.lt_u
          if  ;; label = @4
            local.get 2
            i32.const 16
            i32.add
            local.get 2
            i32.const 24
            i32.add
            i32.const 8
            local.get 4
            call 111
            local.get 3
            i32.const 0
            local.get 2
            i32.load offset=16
            local.tee 3
            local.get 2
            i32.load offset=20
            local.tee 4
            call 48
            drop
            local.get 3
            local.get 4
            call 96
            local.tee 5
            i64.const 256
            i64.ge_u
            br_if 1 (;@3;)
            local.get 5
            i64.const 254
            i64.and
            i64.eqz
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            call 102
            local.get 1
            call 108
            local.get 2
            local.get 1
            call 107
            local.get 2
            i32.load
            i32.eqz
            br_if 3 (;@1;)
            i32.const 1049194
            i32.const 18
            call 39
            unreachable
          end
          i32.const 1048589
          i32.const 14
          call 97
          unreachable
        end
        i32.const 1048589
        i32.const 14
        call 97
        unreachable
      end
      local.get 2
      i32.const 8
      i32.add
      call 89
      local.get 2
      local.get 2
      i32.load8_u offset=12
      i32.store8 offset=28
      local.get 2
      local.get 2
      i32.load offset=8
      i32.store offset=24
      local.get 2
      i32.const 24
      i32.add
      i32.const 1049186
      i32.const 8
      call 62
      local.get 2
      i32.const 24
      i32.add
      i32.const 1049257
      i32.const 19
      call 62
      local.get 2
      i32.load offset=24
      local.get 2
      i32.load8_u offset=28
      call 90
      call 8
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;110;) (type 2) (param i32) (result i32)
    (local i32)
    i32.const 1049292
    i32.const 5
    call 46
    local.tee 1
    local.get 0
    call 79
    local.get 1)
  (func (;111;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 8
    i32.add
    i32.const 0
    local.get 3
    local.get 1
    local.get 2
    call 36
    local.get 4
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 4
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;112;) (type 2) (param i32) (result i32)
    local.get 0
    call 41
    local.tee 0
    call 24
    drop
    local.get 0)
  (func (;113;) (type 6) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1049281
    i32.const 11
    call 46
    local.tee 4
    local.get 0
    call 79
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.sub
            br_table 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          i32.const 0
          local.get 4
          call 92
          br 2 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        call 60
        local.get 3
        local.get 3
        i32.load8_u offset=12
        i32.store8 offset=28
        local.get 3
        local.get 3
        i32.load offset=8
        i32.store offset=24
        i32.const 1
        local.get 3
        i32.const 24
        i32.add
        call 94
        local.get 3
        i32.const 24
        i32.add
        local.get 2
        call 64
        local.get 4
        local.get 3
        i32.load offset=24
        local.get 3
        i32.load8_u offset=28
        call 65
        br 1 (;@1;)
      end
      local.get 3
      i32.const 16
      i32.add
      call 60
      local.get 3
      local.get 3
      i32.load8_u offset=20
      i32.store8 offset=28
      local.get 3
      local.get 3
      i32.load offset=16
      i32.store offset=24
      i32.const 2
      local.get 3
      i32.const 24
      i32.add
      call 94
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      call 64
      local.get 4
      local.get 3
      i32.load offset=24
      local.get 3
      i32.load8_u offset=28
      call 65
    end
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func (;114;) (type 4) (result i32)
    (local i32)
    i32.const 1049140
    i32.const 17
    call 46
    call 91
    call 41
    local.tee 0
    call 25
    drop
    local.get 0)
  (func (;115;) (type 1)
    (local i32)
    call 26
    i32.const 1
    call 55
    i32.const 0
    call 41
    local.tee 0
    call 11
    i32.const 1049140
    i32.const 17
    call 46
    local.get 0
    call 75
    call 21
    drop)
  (func (;116;) (type 1)
    (local i32)
    call 26
    i32.const 1
    call 55
    i32.const 0
    call 52
    local.tee 0
    local.get 0
    call 112
    call 109)
  (func (;117;) (type 1)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      call 27
      i32.eqz
      if  ;; label = @2
        i32.const 1
        call 55
        i32.const 0
        call 52
        local.set 3
        i32.const -11
        local.set 1
        block  ;; label = @3
          i32.const 1059368
          i32.load8_u
          local.tee 2
          if  ;; label = @4
            i32.const -11
            i32.const 2147483647
            local.get 2
            select
            local.set 1
            br 1 (;@3;)
          end
          i32.const 1059368
          i32.const 1
          i32.store8
          i32.const -11
          call 28
        end
        local.get 3
        local.get 3
        call 112
        local.tee 2
        call 109
        local.get 1
        call 114
        call 29
        i32.eqz
        br_if 1 (;@1;)
        i32.const 1049217
        i32.const 40
        call 39
        unreachable
      end
      i32.const 1048654
      i32.const 37
      call 0
      unreachable
    end
    local.get 2
    i32.const 1
    call 40
    local.tee 1
    call 38
    call 113
    local.get 0
    local.get 1
    i32.store offset=76
    local.get 0
    i32.const 40
    i32.add
    local.get 0
    i32.const 76
    i32.add
    local.get 3
    call 100
    local.get 0
    i32.const 8
    i32.add
    local.get 0
    i32.const 40
    i32.add
    call 66
    local.get 0
    i32.const 56
    i32.add
    local.get 2
    call 101
    local.get 0
    local.get 0
    i64.load offset=8
    i64.store offset=40
    local.get 0
    local.get 0
    i64.load offset=16
    i64.store offset=48
    local.get 0
    i32.const 40
    i32.add
    call 58
    unreachable)
  (func (;118;) (type 1)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 26
    i32.const 1
    call 55
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    call 52
    call 112
    call 105
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 57
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;119;) (type 1)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 26
    i32.const 1
    call 55
    local.get 0
    i32.const 8
    i32.add
    call 54
    call 105
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 57
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;120;) (type 1)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    call 26
    i32.const 1
    call 55
    local.get 0
    i32.const 16
    i32.add
    i32.const 0
    call 52
    call 112
    local.tee 1
    call 82
    local.get 0
    i32.load8_u offset=47
    call 106
    i32.const 255
    i32.and
    i32.eq
    if (result i32)  ;; label = @1
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      call 107
      local.get 0
      i32.load offset=12
      local.set 2
      local.get 0
      i32.load offset=8
      i32.const 1
      i32.ne
    else
      i32.const 1
    end
    local.get 2
    call 57
    local.get 0
    i32.const 48
    i32.add
    global.set 0)
  (func (;121;) (type 1)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 26
    call 44
    i32.const 1
    call 55
    i32.const 0
    call 52
    call 112
    local.tee 1
    call 108
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    call 107
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.const 0
      local.get 0
      call 113
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;122;) (type 1)
    (local i32 i32 i32)
    call 26
    call 44
    i32.const 0
    call 55
    call 40
    call 42
    call 41
    local.set 0
    i32.const 1059369
    call 35
    drop
    i32.const 1059369
    local.get 0
    call 30
    local.get 0
    i64.const 0
    call 51
    call 51
    call 31
    drop)
  (func (;123;) (type 1)
    call 26
    i32.const 0
    call 55
    call 114
    call 32)
  (func (;124;) (type 1)
    call 26
    i32.const 0
    call 55
    call 43
    call 13
    drop)
  (func (;125;) (type 1)
    call 26
    i32.const 0
    call 55
    call 106
    i64.extend_i32_u
    i64.const 255
    i64.and
    call 33)
  (func (;126;) (type 1)
    call 26
    i32.const 1
    call 55
    i32.const 0
    call 52
    call 112
    call 13
    drop)
  (func (;127;) (type 1)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    call 26
    i32.const 1
    call 55
    local.get 0
    i32.const 0
    call 52
    call 112
    call 82
    local.get 0
    i64.load8_u offset=31
    call 33
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func (;128;) (type 1)
    call 26
    i32.const 1
    call 55
    i32.const 0
    call 52
    call 102)
  (func (;129;) (type 1)
    call 26
    i32.const 0
    call 55
    i32.const 1049212
    i32.const 5
    call 34)
  (func (;130;) (type 1)
    (local i32)
    call 26
    call 44
    i32.const 2
    call 55
    i32.const 0
    call 52
    local.set 0
    i32.const 1
    i32.const 2
    call 53
    select
    local.get 0
    call 110
    call 92)
  (func (;131;) (type 1)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        call 59
        local.tee 3
        call 91
        local.tee 1
        call 70
        br_if 0 (;@2;)
        local.get 0
        i32.const 24
        i32.add
        local.get 1
        call 38
        call 78
        local.get 0
        i32.const 24
        i32.add
        call 98
        local.set 2
        local.get 0
        i32.const 24
        i32.add
        call 95
        local.set 1
        call 51
        local.set 4
        loop  ;; label = @3
          local.get 1
          if  ;; label = @4
            local.get 4
            local.get 0
            i32.const 24
            i32.add
            call 98
            call 68
            local.get 1
            i32.const -1
            i32.add
            local.set 1
            br 1 (;@3;)
          end
        end
        local.get 0
        i32.load offset=28
        local.get 0
        i32.load offset=24
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        i32.const 40
        i32.add
        local.tee 5
        i32.load8_u
        if  ;; label = @3
          i32.const 1059356
          i32.const 0
          i32.store
          i32.const 1059360
          i32.const 0
          i32.store8
        end
        i32.const -20
        i32.const 0
        i32.const 0
        call 16
        drop
        local.get 3
        i32.const -20
        call 21
        drop
        local.get 0
        i32.const 88
        i32.add
        local.tee 3
        i64.const 0
        i64.store
        local.get 0
        i32.const 80
        i32.add
        local.tee 6
        i64.const 0
        i64.store
        local.get 0
        i32.const 72
        i32.add
        local.tee 7
        i64.const 0
        i64.store
        local.get 0
        i64.const 0
        i64.store offset=64
        local.get 0
        i32.const 16
        i32.add
        local.get 0
        i32.const -64
        i32.sub
        local.get 2
        call 9
        local.tee 1
        call 103
        local.get 2
        i32.const 0
        local.get 0
        i32.load offset=16
        local.get 0
        i32.load offset=20
        call 48
        drop
        local.get 0
        i32.const 48
        i32.add
        local.get 3
        i64.load
        i64.store
        local.get 5
        local.get 6
        i64.load
        i64.store
        local.get 0
        i32.const 32
        i32.add
        local.get 7
        i64.load
        i64.store
        local.get 0
        local.get 0
        i64.load offset=64
        i64.store offset=24
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 22
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 24
            i32.add
            i32.const 1048915
            i32.const 22
            call 135
            br_if 0 (;@4;)
            call 56
            i32.const 1059364
            i32.load
            i32.const 0
            i32.lt_s
            if  ;; label = @5
              i32.const 1048691
              i32.const 17
              call 0
              unreachable
            end
            local.get 0
            i32.const 0
            i32.store offset=60
            local.get 0
            i32.const -64
            i32.sub
            local.get 0
            i32.const 60
            i32.add
            call 49
            i32.const 1059364
            i32.load
            local.get 0
            i32.load offset=60
            i32.gt_s
            if  ;; label = @5
              i32.const 1048708
              i32.const 18
              call 0
              unreachable
            end
            local.get 0
            i32.load offset=64
            local.get 4
            call 63
            local.set 1
            call 56
            local.get 0
            i32.const 0
            i32.store offset=72
            local.get 0
            local.get 1
            i32.store offset=68
            local.get 0
            local.get 4
            i32.store offset=64
            local.get 0
            i32.const -64
            i32.sub
            call 47
            local.set 1
            local.get 0
            i32.load offset=72
            local.get 0
            i32.load offset=68
            i32.lt_u
            if  ;; label = @5
              i32.const 1048708
              i32.const 18
              call 0
              unreachable
            end
            br_if 1 (;@3;)
            local.get 0
            i32.const 8
            i32.add
            local.get 1
            call 107
            local.get 0
            i32.load offset=8
            i32.const 1
            i32.eq
            if  ;; label = @5
              local.get 1
              i32.const 2
              local.get 0
              i32.load offset=12
              call 113
              br 3 (;@2;)
            end
            local.get 1
            i32.const 0
            local.get 1
            call 113
            br 2 (;@2;)
          end
          i32.const 1049086
          i32.const 54
          call 0
          unreachable
        end
        local.get 1
        i32.const 0
        local.get 1
        call 113
      end
      local.get 0
      i32.const 96
      i32.add
      global.set 0
      return
    end
    i32.const 1048589
    i32.const 14
    call 99
    unreachable)
  (func (;132;) (type 1)
    i32.const 1049336
    i32.const 14
    call 0
    unreachable)
  (func (;133;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 2
    i32.const 15
    i32.gt_u
    if  ;; label = @1
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 5
      i32.add
      local.set 3
      local.get 5
      if  ;; label = @2
        local.get 1
        local.set 4
        loop  ;; label = @3
          local.get 0
          local.get 4
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 1
          i32.add
          local.set 4
          local.get 0
          i32.const 1
          i32.add
          local.tee 0
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 3
      local.get 2
      local.get 5
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 6
      i32.add
      local.set 0
      block  ;; label = @2
        local.get 1
        local.get 5
        i32.add
        local.tee 5
        i32.const 3
        i32.and
        local.tee 2
        if  ;; label = @3
          local.get 6
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 5
          i32.const -4
          i32.and
          local.tee 4
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 2
          i32.const 3
          i32.shl
          local.tee 8
          i32.sub
          i32.const 24
          i32.and
          local.set 2
          local.get 4
          i32.load
          local.set 4
          loop  ;; label = @4
            local.get 3
            local.get 4
            local.get 8
            i32.shr_u
            local.get 1
            i32.load
            local.tee 4
            local.get 2
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 3
            i32.const 4
            i32.add
            local.tee 3
            local.get 0
            i32.lt_u
            br_if 0 (;@4;)
          end
          br 1 (;@2;)
        end
        local.get 6
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 5
        local.set 1
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 3
          i32.const 4
          i32.add
          local.tee 3
          local.get 0
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 5
      local.get 6
      i32.add
      local.set 1
    end
    local.get 2
    if  ;; label = @1
      local.get 0
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 0
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 1
        i32.add
        local.tee 0
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end)
  (func (;134;) (type 8) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 0
        i32.load8_u
        local.tee 4
        local.get 1
        i32.load8_u
        local.tee 5
        i32.eq
        if  ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (func (;135;) (type 8) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 134)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1059401))
  (global (;2;) i32 (i32.const 1059408))
  (export "memory" (memory 0))
  (export "init" (func 115))
  (export "canRegister" (func 116))
  (export "register" (func 117))
  (export "resolve" (func 118))
  (export "resolveFromHash" (func 119))
  (export "checkPending" (func 120))
  (export "resetPending" (func 121))
  (export "claim" (func 122))
  (export "getRegistrationCost" (func 123))
  (export "getContractOwner" (func 124))
  (export "getOwnShardId" (func 125))
  (export "nameHash" (func 126))
  (export "nameShard" (func 127))
  (export "validateName" (func 128))
  (export "version" (func 129))
  (export "setFeatureFlag" (func 130))
  (export "callBack" (func 131))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "invalid valueinput too longserializer decode error: argument decode error (): function does not accept ESDT paymenttoo few argumentstoo many argumentswrong number of argumentsMultiESDTNFTTransferESDTNFTTransferESDTTransferCB_CLOSUREinput too shortrecipient address not setinput out of rangestorage decode error: bad array lengthSetUserNameset_user_name_callbackname does not contain suffixname too longerror loading name bytes.elrondwrong suffixname is too shortcharacter not allowedresultcb_name_hashname_hashno callback function with that name exists in contractregistration_costname belongs to another shardregistername already taken1.1.0should pay exactly the registration cost currently disabledvaluevalue_statefeat:\00\00\00Endpoint can only be called by ownerpanic occurred")
  (data (;1;) (i32.const 1049352) "\9c\ff\ff\ff"))
