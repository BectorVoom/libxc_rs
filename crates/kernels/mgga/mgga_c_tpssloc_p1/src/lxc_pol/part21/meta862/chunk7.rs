//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3136/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136<F: Float>(t50846: F, t50848: F, t50853: F, t63911: F, t63914: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F) -> F {
    let t64916 = -F::new(5.0) / F::new(27.0) * t63911 - F::new(2.0) / F::new(27.0) * t63914 + F::new(14.0) / F::new(81.0) * t63918 + t63921 / F::new(9.0) + t63924 / F::new(18.0) + t63927 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t63930 - F::new(8.0) / F::new(9.0) * t63933 - t63936 - F::new(4.0) * t63939 + F::new(80.0) / F::new(81.0) * t50846 + F::new(2.0) / F::new(9.0) * t50848 - F::new(20.0) / F::new(27.0) * t50853;
    t64916
}
