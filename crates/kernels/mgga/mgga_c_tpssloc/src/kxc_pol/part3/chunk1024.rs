//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1024/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1024<F: Float>(t11211: F, t11213: F, t11369: F, t11372: F, t14702: F, t14705: F, t14708: F, t14711: F, t14713: F, t14759: F, t14776: F, t14779: F, t14782: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t14827: F) -> (F,) {
    let t14829 = -t11369 - t11372 + 0.13418888888888888889e0 * t14702 - t14705 + 0.301925e0 * t14708 - t14711 + 0.82785e-1 * t14713 + 0.258925e1 * t14759 + 0.18396666666666666667e0 * t11211 + 0.18396666666666666667e-1 * t11213 + t14776 + 0.36793333333333333333e-1 * t14779 - t14782 - 0.5519e-1 * t14784 - 0.27595e-1 * t14787 - 0.16557e0 * t14790 + 0.33114e0 * t14793 + 0.16557e0 * t14796 + 0.49671e0 * t14799 + 0.19419375e1 * t14802 - 0.412621875e-1 * t14805 + t14827;
    (t14829,)
}
