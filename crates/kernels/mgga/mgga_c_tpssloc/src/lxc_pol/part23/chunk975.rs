//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 975/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk975<F: Float>(t1670: F, t5988: F, t1118: F, t3313: F, t14838: F, t5989: F, t1703: F, t18915: F, t4869: F, t6098: F, t4748: F, t5999: F, t4764: F, t4723: F, t5398: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21723 = t5988 * t1670;
    let t21724 = t21723 * t1118;
    let t21726 = 6.0 * t3313 * t21724;
    let t21728 = 6.0 * t14838 * t5989;
    let t21730 = 0.17544670867903938621e1 * t18915 * t1703;
    let t21732 = 0.35089341735807877242e1 * t4869 * t6098;
    let t21739 = t4748 * t5999;
    let t21741 = t4764 * t5999;
    let t21745 = t4723 * t5398;
    (t21723, t21724, t21726, t21728, t21730, t21732, t21739, t21741, t21745)
}
