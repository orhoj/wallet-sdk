import 'package:wallet_sdk/wallet_sdk.dart';
import 'package:test/test.dart';

void main() {
  group('A group of tests', () {
    final awesome = Awesome();

    setUp(() {
      // Additional setup goes here.
    });

    test('First Test', () {
      expect(awesome.isAwesome, isTrue);
    });

    test("Get key using FFI", () {
      final result = getKeyUsingFfi();
      print(result);
    });
  });
}
