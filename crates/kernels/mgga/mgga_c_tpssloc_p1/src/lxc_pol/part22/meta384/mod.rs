//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1649;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta384<F: Float>(t4571: F, t4644: F, t1031: F, t5904: F, t1022: F, t1539: F, t14211: F, t3071: F, t1023: F, t5685: F, t1616: F, t4343: F, t1009: F, t5848: F, t1011: F, t1019: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18008, t18010, t18014, t18015, t18016, t18020, t18021, t18024) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1649::<F>(t4571, t4644, t1031, t5904, t1022, t1539, t14211, t3071, t1023, t5685, t1616, t4343);
        let (t18025, t18028, t18029, t18030) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1650::<F>(t18024, t3071, t1009, t5848, t1011, t1019);
    (t18008, t18010, t18014, t18015, t18016, t18020, t18021, t18024, t18025, t18028, t18029, t18030)
}
