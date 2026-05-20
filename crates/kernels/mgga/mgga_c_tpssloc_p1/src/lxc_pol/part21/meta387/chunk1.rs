//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1854/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1854<F: Float>(t1025: F, t1041: F, t1046: F, t10873: F, t10883: F, t10952: F, t10965: F, t14077: F, t14080: F, t14084: F, t14085: F, t14093: F, t14099: F, t14103: F, t14107: F, t14114: F, t14117: F, t1622: F, t3039: F, t3048: F, t3117: F, t378: F, t4585: F, t4590: F, t4600: F, t4636: F) -> F {
    let t14120 = -t14077 * t1025 / F::new(288.0) - t14080 * t1046 / F::new(432.0) + t14084 + t14085 * t1046 / F::new(2304.0) + t10965 * t1622 / F::new(4608.0) + t3117 * t4636 / F::new(2304.0) + t1041 * t14093 / F::new(4608.0) - t10952 * t4600 / F::new(1536.0) - t3039 * t14099 / F::new(1536.0) - t3039 * t14103 / F::new(3072.0) + t10883 * t14107 / F::new(3072.0) + t3048 * t4585 / F::new(216.0) - F::new(5.0) / F::new(1296.0) * t3048 * t4590 - t14114 * t378 / F::new(288.0) - t14117 / F::new(13824.0) - t10873 / F::new(648.0);
    t14120
}
