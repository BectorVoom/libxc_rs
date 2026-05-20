//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2142;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2143;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2144;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta692<F: Float>(t28117: F, t81159: F, t1377: F, t6330: F, t1385: F, t22635: F, t26331: F, t26332: F, t5187: F, t19885: F, t90915: F, t91004: F, t28135: F, t6914: F, t1992: F, t550: F, t57607: F, t6976: F, t28168: F, t57704: F, t562: F, t6347: F, t1307: F, t26446: F, t57545: F, t90750: F, t90760: F, t90782: F, t90789: F, t90792: F, t90795: F, t90798: F, t90806: F, t90807: F, t93517: F, t19893: F, t90914: F, t1799: F, t1834: F, t1352: F, t22633: F, t19743: F, t3807: F, t20014: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t96920, t96925, t96929, t96935) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2142::<F>(t28117, t81159, t1377, t6330, t1385, t22635, t26331, t26332, t5187, t19885, t90915, t91004);
        let (t96937, t96941, t96945, t96949, t96951, t96954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2143::<F>(t28135, t6914, t1992, t550, t57607, t6976, t28168, t57704, t562, t6347, t1307, t26331, t26446);
        let t96960 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2144::<F>(t1992, t550, t57545, t6976, t90750, t90760, t90782, t90789, t90792, t90795, t90798, t90806, t90807, t93517, t96935, t96937, t96941, t96945, t96949, t96954);
        let (t96962, t96964, t96967, t96972, t96976, t96979) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2145::<F>(t19893, t90914, t90915, t1799, t1834, t1352, t22633, t6976, t96951, t19743, t3807, t1992, t20014);
    (t96920, t96925, t96929, t96960, t96962, t96964, t96967, t96972, t96976, t96979)
}
