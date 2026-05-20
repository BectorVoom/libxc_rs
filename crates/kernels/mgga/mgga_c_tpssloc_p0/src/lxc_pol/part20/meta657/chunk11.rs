//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2439/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439<F: Float>(t13969: F, t14102: F, t3039: F, t10876: F, t13990: F, t14134: F, t3048: F, t1025: F, t10957: F, t14093: F, t42735: F, t42752: F, t43094: F, t43097: F, t4636: F, t49866: F, t49872: F, t49873: F, t49877: F) -> F {
    let t49884 = t3039 * t13969 * t14102;
    let t49887 = t10876 * t13969 * t13990;
    let t49889 = t3048 * t14134;
    let t49891 = t42735 / F::new(4608.0) + t42752 / F::new(5184.0) + t49866 * t1025 / F::new(1024.0) - t49872 - t49873 / F::new(576.0) + F::new(19.0) / F::new(864.0) * t10957 * t4636 - t49877 / F::new(216.0) - t3048 * t14093 / F::new(288.0) + t43094 / F::new(768.0) - t43097 / F::new(1536.0) - t49884 / F::new(1536.0) - t49887 / F::new(256.0) + t49889 / F::new(108.0);
    t49891
}
