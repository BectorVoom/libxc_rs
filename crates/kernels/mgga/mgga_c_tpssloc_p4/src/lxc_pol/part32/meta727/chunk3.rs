//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2356/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2356<F: Float>(t104995: F, t104996: F, t1266: F, t12725: F, t19456: F, t27879: F, t29486: F, t4028: F, t574: F, t7989: F, t96784: F, t96786: F, t96789: F, t96792: F, t96796: F, t96799: F, t96802: F, t96805: F, t96807: F, t96813: F, t96815: F, t96818: F, t96827: F, t96829: F) -> F {
    let t105005 = -t96784 - t96786 - t96789 + t96792 + t96796 + t96799 - t96802 + t96805 - t96807 - t96813 - t96815 - t96818 + t96827 - t29486 * t1266 + (t104995 + t104996) * t574 - t96829 - F::cast_from(4.0_f64) * t19456 * t7989 - F::cast_from(4.0_f64) * t4028 * t27879 - F::cast_from(4.0_f64) * t12725 * t7989;
    t105005
}
