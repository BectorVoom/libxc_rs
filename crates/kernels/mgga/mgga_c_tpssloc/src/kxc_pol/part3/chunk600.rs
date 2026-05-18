//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 600/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk600<F: Float>(t2906: F, t951: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F, F, F) {
    let t2907 = t2906 * t951;
    let t2912 = F::new(0.40256666666666666667e0) * t2764;
    let t2919 = F::new(0.137975e0) * t2822;
    let t2924 = -F::new(0.1294625e1) * t2800 + F::new(0.258925e1) * t2808 + t2912 + F::new(0.20128333333333333334e0) * t2766 - F::new(0.20128333333333333333e0) * t2773 + F::new(0.60385e0) * t2778 - F::new(0.301925e0) * t2782 + F::new(0.82524375e-1) * t2816 + F::new(0.16504875e0) * t2818 + t2919 + F::new(0.11038e0) * t2824 - F::new(0.27595e-1) * t2828 + F::new(0.16557e0) * t2831 - F::new(0.82785e-1) * t2834;
    (t2907, t2912, t2919, t2924)
}
