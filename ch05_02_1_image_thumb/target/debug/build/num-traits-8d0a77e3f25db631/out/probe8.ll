; ModuleID = 'probe8.33aa9ee3-cgu.0'
source_filename = "probe8.33aa9ee3-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; core::intrinsics::const_eval_select
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core10intrinsics17const_eval_select17hefbfa8fc8fb45dd7E(double %arg) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = alloca { i8*, i32 }, align 8
; invoke core::ops::function::FnOnce::call_once
  %1 = invoke i64 @_ZN4core3ops8function6FnOnce9call_once17hf4f08373e2bc1f12E(double %arg)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

cleanup:                                          ; preds = %start
  %2 = landingpad { i8*, i32 }
          cleanup
  %3 = extractvalue { i8*, i32 } %2, 0
  %4 = extractvalue { i8*, i32 } %2, 1
  %5 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %3, i8** %5, align 8
  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %4, i32* %6, align 8
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  %7 = bitcast { i8*, i32 }* %0 to i8**
  %8 = load i8*, i8** %7, align 8
  %9 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  %10 = load i32, i32* %9, align 8
  %11 = insertvalue { i8*, i32 } undef, i8* %8, 0
  %12 = insertvalue { i8*, i32 } %11, i32 %10, 1
  resume { i8*, i32 } %12

bb2:                                              ; preds = %bb1
  ret i64 %1
}

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17ha3c77bdcb943c952E"(double %self) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_bits
  %_2 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17hc15f8e13b4140273E"(double %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl u64>::to_ne_bytes
  %2 = call i64 @"_ZN4core3num21_$LT$impl$u20$u64$GT$11to_ne_bytes17h8988a7f3553693f2E"(i64 %_2)
  store i64 %2, i64* %0, align 8
  %3 = bitcast [8 x i8]* %1 to i8*
  %4 = bitcast i64* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 8 %4, i64 8, i1 false)
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast [8 x i8]* %1 to i64*
  %6 = load i64, i64* %5, align 1
  ret i64 %6
}

; core::f64::<impl f64>::to_bits
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits17hc15f8e13b4140273E"(double %self) unnamed_addr #0 {
start:
  %_3 = alloca double, align 8
  store double %self, double* %_3, align 8
  %0 = load double, double* %_3, align 8
; call core::intrinsics::const_eval_select
  %1 = call i64 @_ZN4core10intrinsics17const_eval_select17hefbfa8fc8fb45dd7E(double %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::f64::<impl f64>::to_bits::{{closure}}
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h26871754f0ddc64fE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %rt) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = bitcast double %rt to i64
  store i64 %1, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %2
}

; core::num::<impl u64>::to_ne_bytes
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3num21_$LT$impl$u20$u64$GT$11to_ne_bytes17h8988a7f3553693f2E"(i64 %self) unnamed_addr #0 {
start:
  %0 = alloca [8 x i8], align 1
  %1 = bitcast [8 x i8]* %0 to i64*
  store i64 %self, i64* %1, align 1
  br label %bb1

bb1:                                              ; preds = %start
  %2 = bitcast [8 x i8]* %0 to i64*
  %3 = load i64, i64* %2, align 1
  ret i64 %3
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3ops8function6FnOnce9call_once17hf4f08373e2bc1f12E(double %0) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_2 = alloca double, align 8
  %_1 = alloca %"[closure@core::f64::<impl f64>::to_bits::{closure#0}]", align 1
  store double %0, double* %_2, align 8
  %2 = load double, double* %_2, align 8
; invoke core::f64::<impl f64>::to_bits::{{closure}}
  %3 = invoke i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits28_$u7b$$u7b$closure$u7d$$u7d$17h26871754f0ddc64fE"(%"[closure@core::f64::<impl f64>::to_bits::{closure#0}]"* align 1 %_1, double %2)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

cleanup:                                          ; preds = %start
  %4 = landingpad { i8*, i32 }
          cleanup
  %5 = extractvalue { i8*, i32 } %4, 0
  %6 = extractvalue { i8*, i32 } %4, 1
  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %5, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %6, i32* %8, align 8
  br label %bb3

bb1:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %bb3
  %9 = bitcast { i8*, i32 }* %1 to i8**
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %12 = load i32, i32* %11, align 8
  %13 = insertvalue { i8*, i32 } undef, i8* %10, 0
  %14 = insertvalue { i8*, i32 } %13, i32 %12, 1
  resume { i8*, i32 } %14

bb2:                                              ; preds = %bb1
  ret i64 %3
}

; probe8::probe
; Function Attrs: uwtable
define void @_ZN6probe85probe17hbdeaefc5064c12dcE() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  %1 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17ha3c77bdcb943c952E"(double 3.140000e+00)
  store i64 %1, i64* %0, align 8
  %2 = bitcast [8 x i8]* %_1 to i8*
  %3 = bitcast i64* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 8 %3, i64 8, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { argmemonly nofree nounwind willreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
