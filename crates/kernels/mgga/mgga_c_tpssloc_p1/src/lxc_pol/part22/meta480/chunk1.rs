//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1883/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1883<F: Float>(t21013: F, t218: F, t1528: F, t17052: F, t17090: F, t17092: F, t21034: F, t21036: F, t21038: F, t21050: F, t21054: F, t21061: F, t259: F, t4147: F, t4268: F, t5637: F, t5658: F, t855: F) -> (F, F) {
    let t21064 = t218 * t21013;
    let t21066 = -F::new(3.0) * t1528 * t17052 - F::new(3.0) * t1528 * t17090 - F::new(6.0) * t1528 * t17092 - t21034 * t855 + t21036 * t259 + F::new(3.0) * t21038 * t259 - F::new(6.0) * t21050 * t855 + F::new(6.0) * t21054 * t855 + F::new(3.0) * t21061 * t259 + t21064 * t259 + F::new(6.0) * t4147 * t5637 - F::new(3.0) * t4147 * t5658 + F::new(6.0) * t4268 * t5637 - F::new(3.0) * t4268 * t5658;
    (t21064, t21066)
}
