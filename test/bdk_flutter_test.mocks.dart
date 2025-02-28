// Mocks generated by Mockito 5.4.2 from annotations
// in bdk_flutter/test/bdk_flutter_test.dart.
// Do not manually edit this file.

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'dart:async' as _i4;

import 'package:bdk_flutter/src/generated/bridge_definitions.dart' as _i2;
import 'package:bdk_flutter/src/root.dart' as _i3;
import 'package:mockito/mockito.dart' as _i1;

// ignore_for_file: type=lint
// ignore_for_file: avoid_redundant_argument_values
// ignore_for_file: avoid_setters_without_getters
// ignore_for_file: comment_references
// ignore_for_file: implementation_imports
// ignore_for_file: invalid_use_of_visible_for_testing_member
// ignore_for_file: prefer_const_constructors
// ignore_for_file: unnecessary_parenthesis
// ignore_for_file: camel_case_types
// ignore_for_file: subtype_of_sealed_class

class _FakeAddressInfo_0 extends _i1.SmartFake implements _i2.AddressInfo {
  _FakeAddressInfo_0(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeBalance_1 extends _i1.SmartFake implements _i2.Balance {
  _FakeBalance_1(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakePartiallySignedTransaction_2 extends _i1.SmartFake
    implements _i3.PartiallySignedTransaction {
  _FakePartiallySignedTransaction_2(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeFeeRate_3 extends _i1.SmartFake implements _i3.FeeRate {
  _FakeFeeRate_3(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeDescriptorPublicKey_4 extends _i1.SmartFake
    implements _i3.DescriptorPublicKey {
  _FakeDescriptorPublicKey_4(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeDescriptorSecretKey_5 extends _i1.SmartFake
    implements _i3.DescriptorSecretKey {
  _FakeDescriptorSecretKey_5(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeTransaction_6 extends _i1.SmartFake implements _i3.Transaction {
  _FakeTransaction_6(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeTxBuilder_7 extends _i1.SmartFake implements _i3.TxBuilder {
  _FakeTxBuilder_7(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeTxBuilderResult_8 extends _i1.SmartFake
    implements _i3.TxBuilderResult {
  _FakeTxBuilderResult_8(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeBumpFeeTxBuilder_9 extends _i1.SmartFake
    implements _i3.BumpFeeTxBuilder {
  _FakeBumpFeeTxBuilder_9(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeScript_10 extends _i1.SmartFake implements _i3.Script {
  _FakeScript_10(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

/// A class which mocks [Wallet].
///
/// See the documentation for Mockito's code generation for more information.
class MockWallet extends _i1.Mock implements _i3.Wallet {
  @override
  _i4.Future<_i2.AddressInfo> getAddress(
          {required _i2.AddressIndex? addressIndex}) =>
      (super.noSuchMethod(
        Invocation.method(
          #getAddress,
          [],
          {#addressIndex: addressIndex},
        ),
        returnValue: _i4.Future<_i2.AddressInfo>.value(_FakeAddressInfo_0(
          this,
          Invocation.method(
            #getAddress,
            [],
            {#addressIndex: addressIndex},
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i2.AddressInfo>.value(_FakeAddressInfo_0(
          this,
          Invocation.method(
            #getAddress,
            [],
            {#addressIndex: addressIndex},
          ),
        )),
      ) as _i4.Future<_i2.AddressInfo>);

  @override
  _i4.Future<_i2.Balance> getBalance() => (super.noSuchMethod(
        Invocation.method(
          #getBalance,
          [],
        ),
        returnValue: _i4.Future<_i2.Balance>.value(_FakeBalance_1(
          this,
          Invocation.method(
            #getBalance,
            [],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i2.Balance>.value(_FakeBalance_1(
          this,
          Invocation.method(
            #getBalance,
            [],
          ),
        )),
      ) as _i4.Future<_i2.Balance>);

  @override
  _i4.Future<_i2.Network> network() => (super.noSuchMethod(
        Invocation.method(
          #network,
          [],
        ),
        returnValue: _i4.Future<_i2.Network>.value(_i2.Network.Testnet),
        returnValueForMissingStub:
            _i4.Future<_i2.Network>.value(_i2.Network.Testnet),
      ) as _i4.Future<_i2.Network>);

  @override
  _i4.Future<List<_i2.LocalUtxo>> listUnspent() => (super.noSuchMethod(
        Invocation.method(
          #listUnspent,
          [],
        ),
        returnValue: _i4.Future<List<_i2.LocalUtxo>>.value(<_i2.LocalUtxo>[]),
        returnValueForMissingStub:
            _i4.Future<List<_i2.LocalUtxo>>.value(<_i2.LocalUtxo>[]),
      ) as _i4.Future<List<_i2.LocalUtxo>>);

  @override
  _i4.Future<dynamic> sync(_i3.Blockchain? blockchain) => (super.noSuchMethod(
        Invocation.method(
          #sync,
          [blockchain],
        ),
        returnValue: _i4.Future<dynamic>.value(),
        returnValueForMissingStub: _i4.Future<dynamic>.value(),
      ) as _i4.Future<dynamic>);

  @override
  _i4.Future<List<_i2.TransactionDetails>> listTransactions() =>
      (super.noSuchMethod(
        Invocation.method(
          #listTransactions,
          [],
        ),
        returnValue: _i4.Future<List<_i2.TransactionDetails>>.value(
            <_i2.TransactionDetails>[]),
        returnValueForMissingStub:
            _i4.Future<List<_i2.TransactionDetails>>.value(
                <_i2.TransactionDetails>[]),
      ) as _i4.Future<List<_i2.TransactionDetails>>);

  @override
  _i4.Future<_i3.PartiallySignedTransaction> sign(
          _i3.PartiallySignedTransaction? psbt) =>
      (super.noSuchMethod(
        Invocation.method(
          #sign,
          [psbt],
        ),
        returnValue: _i4.Future<_i3.PartiallySignedTransaction>.value(
            _FakePartiallySignedTransaction_2(
          this,
          Invocation.method(
            #sign,
            [psbt],
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i3.PartiallySignedTransaction>.value(
                _FakePartiallySignedTransaction_2(
          this,
          Invocation.method(
            #sign,
            [psbt],
          ),
        )),
      ) as _i4.Future<_i3.PartiallySignedTransaction>);
}

/// A class which mocks [Blockchain].
///
/// See the documentation for Mockito's code generation for more information.
class MockBlockchain extends _i1.Mock implements _i3.Blockchain {
  @override
  _i4.Future<String> getBlockHash(int? height) => (super.noSuchMethod(
        Invocation.method(
          #getBlockHash,
          [height],
        ),
        returnValue: _i4.Future<String>.value(''),
        returnValueForMissingStub: _i4.Future<String>.value(''),
      ) as _i4.Future<String>);

  @override
  _i4.Future<int> getHeight() => (super.noSuchMethod(
        Invocation.method(
          #getHeight,
          [],
        ),
        returnValue: _i4.Future<int>.value(0),
        returnValueForMissingStub: _i4.Future<int>.value(0),
      ) as _i4.Future<int>);

  @override
  _i4.Future<_i3.FeeRate> estimateFee(int? target) => (super.noSuchMethod(
        Invocation.method(
          #estimateFee,
          [target],
        ),
        returnValue: _i4.Future<_i3.FeeRate>.value(_FakeFeeRate_3(
          this,
          Invocation.method(
            #estimateFee,
            [target],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.FeeRate>.value(_FakeFeeRate_3(
          this,
          Invocation.method(
            #estimateFee,
            [target],
          ),
        )),
      ) as _i4.Future<_i3.FeeRate>);

  @override
  _i4.Future<String?> broadcast(_i3.Transaction? tx) => (super.noSuchMethod(
        Invocation.method(
          #broadcast,
          [tx],
        ),
        returnValue: _i4.Future<String?>.value(),
        returnValueForMissingStub: _i4.Future<String?>.value(),
      ) as _i4.Future<String?>);
}

/// A class which mocks [DescriptorSecretKey].
///
/// See the documentation for Mockito's code generation for more information.
class MockDescriptorSecretKey extends _i1.Mock
    implements _i3.DescriptorSecretKey {
  @override
  _i4.Future<_i3.DescriptorPublicKey> asPublic() => (super.noSuchMethod(
        Invocation.method(
          #asPublic,
          [],
        ),
        returnValue: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #asPublic,
            [],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #asPublic,
            [],
          ),
        )),
      ) as _i4.Future<_i3.DescriptorPublicKey>);

  @override
  String asString() => (super.noSuchMethod(
        Invocation.method(
          #asString,
          [],
        ),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  _i4.Future<_i3.DescriptorSecretKey> derive(
          _i3.DerivationPath? derivationPath) =>
      (super.noSuchMethod(
        Invocation.method(
          #derive,
          [derivationPath],
        ),
        returnValue: _i4.Future<_i3.DescriptorSecretKey>.value(
            _FakeDescriptorSecretKey_5(
          this,
          Invocation.method(
            #derive,
            [derivationPath],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.DescriptorSecretKey>.value(
            _FakeDescriptorSecretKey_5(
          this,
          Invocation.method(
            #derive,
            [derivationPath],
          ),
        )),
      ) as _i4.Future<_i3.DescriptorSecretKey>);

  @override
  _i4.Future<_i3.DescriptorSecretKey> extend(
          _i3.DerivationPath? derivationPath) =>
      (super.noSuchMethod(
        Invocation.method(
          #extend,
          [derivationPath],
        ),
        returnValue: _i4.Future<_i3.DescriptorSecretKey>.value(
            _FakeDescriptorSecretKey_5(
          this,
          Invocation.method(
            #extend,
            [derivationPath],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.DescriptorSecretKey>.value(
            _FakeDescriptorSecretKey_5(
          this,
          Invocation.method(
            #extend,
            [derivationPath],
          ),
        )),
      ) as _i4.Future<_i3.DescriptorSecretKey>);

  @override
  _i4.Future<List<int>> secretBytes() => (super.noSuchMethod(
        Invocation.method(
          #secretBytes,
          [],
        ),
        returnValue: _i4.Future<List<int>>.value(<int>[]),
        returnValueForMissingStub: _i4.Future<List<int>>.value(<int>[]),
      ) as _i4.Future<List<int>>);
}

/// A class which mocks [DescriptorPublicKey].
///
/// See the documentation for Mockito's code generation for more information.
class MockDescriptorPublicKey extends _i1.Mock
    implements _i3.DescriptorPublicKey {
  @override
  String asString() => (super.noSuchMethod(
        Invocation.method(
          #asString,
          [],
        ),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  _i4.Future<_i3.DescriptorPublicKey> derive(
          _i3.DerivationPath? derivationPath) =>
      (super.noSuchMethod(
        Invocation.method(
          #derive,
          [derivationPath],
        ),
        returnValue: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #derive,
            [derivationPath],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #derive,
            [derivationPath],
          ),
        )),
      ) as _i4.Future<_i3.DescriptorPublicKey>);

  @override
  _i4.Future<_i3.DescriptorPublicKey> extend(
          _i3.DerivationPath? derivationPath) =>
      (super.noSuchMethod(
        Invocation.method(
          #extend,
          [derivationPath],
        ),
        returnValue: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #extend,
            [derivationPath],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.DescriptorPublicKey>.value(
            _FakeDescriptorPublicKey_4(
          this,
          Invocation.method(
            #extend,
            [derivationPath],
          ),
        )),
      ) as _i4.Future<_i3.DescriptorPublicKey>);
}

/// A class which mocks [PartiallySignedTransaction].
///
/// See the documentation for Mockito's code generation for more information.
class MockPartiallySignedTransaction extends _i1.Mock
    implements _i3.PartiallySignedTransaction {
  @override
  String get psbtBase64 => (super.noSuchMethod(
        Invocation.getter(#psbtBase64),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  _i4.Future<_i3.PartiallySignedTransaction> combine(
          _i3.PartiallySignedTransaction? other) =>
      (super.noSuchMethod(
        Invocation.method(
          #combine,
          [other],
        ),
        returnValue: _i4.Future<_i3.PartiallySignedTransaction>.value(
            _FakePartiallySignedTransaction_2(
          this,
          Invocation.method(
            #combine,
            [other],
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i3.PartiallySignedTransaction>.value(
                _FakePartiallySignedTransaction_2(
          this,
          Invocation.method(
            #combine,
            [other],
          ),
        )),
      ) as _i4.Future<_i3.PartiallySignedTransaction>);

  @override
  _i4.Future<_i3.Transaction> extractTx() => (super.noSuchMethod(
        Invocation.method(
          #extractTx,
          [],
        ),
        returnValue: _i4.Future<_i3.Transaction>.value(_FakeTransaction_6(
          this,
          Invocation.method(
            #extractTx,
            [],
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i3.Transaction>.value(_FakeTransaction_6(
          this,
          Invocation.method(
            #extractTx,
            [],
          ),
        )),
      ) as _i4.Future<_i3.Transaction>);

  @override
  _i4.Future<int?> feeAmount() => (super.noSuchMethod(
        Invocation.method(
          #feeAmount,
          [],
        ),
        returnValue: _i4.Future<int?>.value(),
        returnValueForMissingStub: _i4.Future<int?>.value(),
      ) as _i4.Future<int?>);

  @override
  _i4.Future<_i3.FeeRate?> feeRate() => (super.noSuchMethod(
        Invocation.method(
          #feeRate,
          [],
        ),
        returnValue: _i4.Future<_i3.FeeRate?>.value(),
        returnValueForMissingStub: _i4.Future<_i3.FeeRate?>.value(),
      ) as _i4.Future<_i3.FeeRate?>);

  @override
  _i4.Future<String> serialize() => (super.noSuchMethod(
        Invocation.method(
          #serialize,
          [],
        ),
        returnValue: _i4.Future<String>.value(''),
        returnValueForMissingStub: _i4.Future<String>.value(''),
      ) as _i4.Future<String>);

  @override
  _i4.Future<String> txId() => (super.noSuchMethod(
        Invocation.method(
          #txId,
          [],
        ),
        returnValue: _i4.Future<String>.value(''),
        returnValueForMissingStub: _i4.Future<String>.value(''),
      ) as _i4.Future<String>);
}

/// A class which mocks [TxBuilder].
///
/// See the documentation for Mockito's code generation for more information.
class MockTxBuilder extends _i1.Mock implements _i3.TxBuilder {
  @override
  _i3.TxBuilder addData({required List<int>? data}) => (super.noSuchMethod(
        Invocation.method(
          #addData,
          [],
          {#data: data},
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addData,
            [],
            {#data: data},
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addData,
            [],
            {#data: data},
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder addRecipient(
    _i3.Script? script,
    int? amount,
  ) =>
      (super.noSuchMethod(
        Invocation.method(
          #addRecipient,
          [
            script,
            amount,
          ],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addRecipient,
            [
              script,
              amount,
            ],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addRecipient,
            [
              script,
              amount,
            ],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder unSpendable(List<_i2.OutPoint>? outpoints) =>
      (super.noSuchMethod(
        Invocation.method(
          #unSpendable,
          [outpoints],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #unSpendable,
            [outpoints],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #unSpendable,
            [outpoints],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder addUtxo(_i2.OutPoint? outpoint) => (super.noSuchMethod(
        Invocation.method(
          #addUtxo,
          [outpoint],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUtxo,
            [outpoint],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUtxo,
            [outpoint],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder addUtxos(List<_i2.OutPoint>? outpoints) => (super.noSuchMethod(
        Invocation.method(
          #addUtxos,
          [outpoints],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUtxos,
            [outpoints],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUtxos,
            [outpoints],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder doNotSpendChange() => (super.noSuchMethod(
        Invocation.method(
          #doNotSpendChange,
          [],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #doNotSpendChange,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #doNotSpendChange,
            [],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder drainWallet() => (super.noSuchMethod(
        Invocation.method(
          #drainWallet,
          [],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #drainWallet,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #drainWallet,
            [],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder drainTo(String? address) => (super.noSuchMethod(
        Invocation.method(
          #drainTo,
          [address],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #drainTo,
            [address],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #drainTo,
            [address],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder enableRbfWithSequence(int? nSequence) => (super.noSuchMethod(
        Invocation.method(
          #enableRbfWithSequence,
          [nSequence],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #enableRbfWithSequence,
            [nSequence],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #enableRbfWithSequence,
            [nSequence],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder enableRbf() => (super.noSuchMethod(
        Invocation.method(
          #enableRbf,
          [],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #enableRbf,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #enableRbf,
            [],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder feeAbsolute(int? feeAmount) => (super.noSuchMethod(
        Invocation.method(
          #feeAbsolute,
          [feeAmount],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #feeAbsolute,
            [feeAmount],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #feeAbsolute,
            [feeAmount],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder feeRate(double? satPerVbyte) => (super.noSuchMethod(
        Invocation.method(
          #feeRate,
          [satPerVbyte],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #feeRate,
            [satPerVbyte],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #feeRate,
            [satPerVbyte],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder setRecipients(List<_i2.ScriptAmount>? recipients) =>
      (super.noSuchMethod(
        Invocation.method(
          #setRecipients,
          [recipients],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #setRecipients,
            [recipients],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #setRecipients,
            [recipients],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder manuallySelectedOnly() => (super.noSuchMethod(
        Invocation.method(
          #manuallySelectedOnly,
          [],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #manuallySelectedOnly,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #manuallySelectedOnly,
            [],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder addUnSpendable(_i2.OutPoint? unSpendable) =>
      (super.noSuchMethod(
        Invocation.method(
          #addUnSpendable,
          [unSpendable],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUnSpendable,
            [unSpendable],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #addUnSpendable,
            [unSpendable],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i3.TxBuilder onlySpendChange() => (super.noSuchMethod(
        Invocation.method(
          #onlySpendChange,
          [],
        ),
        returnValue: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #onlySpendChange,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeTxBuilder_7(
          this,
          Invocation.method(
            #onlySpendChange,
            [],
          ),
        ),
      ) as _i3.TxBuilder);

  @override
  _i4.Future<_i3.TxBuilderResult> finish(_i3.Wallet? wallet) =>
      (super.noSuchMethod(
        Invocation.method(
          #finish,
          [wallet],
        ),
        returnValue:
            _i4.Future<_i3.TxBuilderResult>.value(_FakeTxBuilderResult_8(
          this,
          Invocation.method(
            #finish,
            [wallet],
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i3.TxBuilderResult>.value(_FakeTxBuilderResult_8(
          this,
          Invocation.method(
            #finish,
            [wallet],
          ),
        )),
      ) as _i4.Future<_i3.TxBuilderResult>);
}

/// A class which mocks [BumpFeeTxBuilder].
///
/// See the documentation for Mockito's code generation for more information.
class MockBumpFeeTxBuilder extends _i1.Mock implements _i3.BumpFeeTxBuilder {
  @override
  String get txid => (super.noSuchMethod(
        Invocation.getter(#txid),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  double get feeRate => (super.noSuchMethod(
        Invocation.getter(#feeRate),
        returnValue: 0.0,
        returnValueForMissingStub: 0.0,
      ) as double);

  @override
  _i3.BumpFeeTxBuilder allowShrinking(String? address) => (super.noSuchMethod(
        Invocation.method(
          #allowShrinking,
          [address],
        ),
        returnValue: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #allowShrinking,
            [address],
          ),
        ),
        returnValueForMissingStub: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #allowShrinking,
            [address],
          ),
        ),
      ) as _i3.BumpFeeTxBuilder);

  @override
  _i3.BumpFeeTxBuilder enableRbf() => (super.noSuchMethod(
        Invocation.method(
          #enableRbf,
          [],
        ),
        returnValue: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #enableRbf,
            [],
          ),
        ),
        returnValueForMissingStub: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #enableRbf,
            [],
          ),
        ),
      ) as _i3.BumpFeeTxBuilder);

  @override
  _i3.BumpFeeTxBuilder enableRbfWithSequence(int? nSequence) =>
      (super.noSuchMethod(
        Invocation.method(
          #enableRbfWithSequence,
          [nSequence],
        ),
        returnValue: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #enableRbfWithSequence,
            [nSequence],
          ),
        ),
        returnValueForMissingStub: _FakeBumpFeeTxBuilder_9(
          this,
          Invocation.method(
            #enableRbfWithSequence,
            [nSequence],
          ),
        ),
      ) as _i3.BumpFeeTxBuilder);

  @override
  _i4.Future<_i3.TxBuilderResult> finish(_i3.Wallet? wallet) =>
      (super.noSuchMethod(
        Invocation.method(
          #finish,
          [wallet],
        ),
        returnValue:
            _i4.Future<_i3.TxBuilderResult>.value(_FakeTxBuilderResult_8(
          this,
          Invocation.method(
            #finish,
            [wallet],
          ),
        )),
        returnValueForMissingStub:
            _i4.Future<_i3.TxBuilderResult>.value(_FakeTxBuilderResult_8(
          this,
          Invocation.method(
            #finish,
            [wallet],
          ),
        )),
      ) as _i4.Future<_i3.TxBuilderResult>);
}

/// A class which mocks [Script].
///
/// See the documentation for Mockito's code generation for more information.
class MockScript extends _i1.Mock implements _i3.Script {}

/// A class which mocks [Address].
///
/// See the documentation for Mockito's code generation for more information.
class MockAddress extends _i1.Mock implements _i3.Address {
  @override
  _i4.Future<_i3.Script> scriptPubKey() => (super.noSuchMethod(
        Invocation.method(
          #scriptPubKey,
          [],
        ),
        returnValue: _i4.Future<_i3.Script>.value(_FakeScript_10(
          this,
          Invocation.method(
            #scriptPubKey,
            [],
          ),
        )),
        returnValueForMissingStub: _i4.Future<_i3.Script>.value(_FakeScript_10(
          this,
          Invocation.method(
            #scriptPubKey,
            [],
          ),
        )),
      ) as _i4.Future<_i3.Script>);
}

/// A class which mocks [DerivationPath].
///
/// See the documentation for Mockito's code generation for more information.
class MockDerivationPath extends _i1.Mock implements _i3.DerivationPath {}

/// A class which mocks [FeeRate].
///
/// See the documentation for Mockito's code generation for more information.
class MockFeeRate extends _i1.Mock implements _i3.FeeRate {
  @override
  double asSatPerVb() => (super.noSuchMethod(
        Invocation.method(
          #asSatPerVb,
          [],
        ),
        returnValue: 0.0,
        returnValueForMissingStub: 0.0,
      ) as double);
}
