import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'package:ffi/ffi.dart';

// rust/native function
typedef NativePlayFunction = ffi.Void Function(ffi.Pointer<Utf8>);
// dart function
typedef PlayFunction = void Function(ffi.Pointer<Utf8>);

void main(List<String> arguments) {
  ffi.DynamicLibrary dl =
      ffi.DynamicLibrary.open('target/debug/libaudio_lib.dylib');
  final PlayFunction play = dl
      .lookup<ffi.NativeFunction<NativePlayFunction>>("play_for_ffi")
      .asFunction();
  final ffi.Pointer<Utf8> path = "data/beep-01a.wav".toNativeUtf8().cast();
  play(path);
}
