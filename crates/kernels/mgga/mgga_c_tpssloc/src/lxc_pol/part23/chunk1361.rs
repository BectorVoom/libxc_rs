//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1361/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361<F: Float>(t6460: F, t40343: F, t40347: F, t40350: F, t54633: F, t54639: F, t56465: F, t56469: F, t56484: F, t56491: F, t74702: F, t74724: F, t74741: F, t74745: F, t6330: F, t1315: F, t16101: F, t1799: F, t19781: F, t210: F, t214: F, t221: F, t3733: F, t40025: F, t40401: F, t40422: F, t5195: F, t54663: F, t54725: F, t56535: F, t56539: F, t6347: F, t74726: F, t74747: F, t74756: F, t79921: F, t79984: F) -> (F, F, F, F) {
    let t79993 = t6460 * t6460;
    let t80019 = -t40343 + t40347 + t40350 + 0.13148148148148148148e0 * t54633 + 0.22469135802469135801e0 * t54639 - 0.29999999999999999998e-1 * t56465 + 0.99999999999999999996e-2 * t56469 + 0.33333333333333333332e-2 * t74702 - 0.29999999999999999998e-1 * t74724 + 0.23333333333333333332e0 * t56484 - 0.77777777777777777775e-1 * t56491 + 0.18666666666666666665e0 * t74741 + 0.39999999999999999998e-1 * t74745;
    let t80021 = t6330 * t6330;
    let t80047 = 0.15555555555555555555e-1 * t74747 - t40401 + t40422 + 0.99999999999999999995e-1 * t40025 * t210 * t214 * t80021 - 0.79999999999999999997e-1 * t54663 - 0.13999999999999999999e0 * t74756 + 0.94999999999999999997e-1 * t56535 - 0.31666666666666666666e-1 * t56539 + 0.11111111111111111111e-2 * t54725 - 0.16666666666666666666e-2 * t1315 * t210 * t214 * t79984 + 0.14999999999999999999e-1 * t3733 * t210 * t214 * t79921 + 0.19999999999999999999e-1 * t5195 * t221 * t74726 * t1799 - 0.11999999999999999999e0 * t16101 * t221 * t19781 * t6347;
    (t79993, t80019, t80021, t80047)
}
