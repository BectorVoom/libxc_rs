//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2613/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2613<F: Float>(t11678: F, t11697: F, t22279: F, t1227: F, t15453: F, t1735: F, t18206: F, t19077: F, t22258: F, t3490: F, t3577: F, t45020: F, t45128: F, t4582: F, t4972: F, t52836: F, t53079: F, t53097: F, t53099: F, t66268: F, t66273: F, t66276: F, t66324: F, t70316: F, t70339: F) -> F {
    let t72936 = t11678 * t11697 * t22279;
    let t72938 = t66268 / F::new(216.0) + t53079 / F::new(3456.0) + t53097 + t66273 / F::new(54.0) - t66276 / F::new(288.0) + t53099 / F::new(3456.0) - t3490 * t22258 / F::new(768.0) - t1227 * t4582 * t4972 * t70316 / F::new(768.0) + t45020 / F::new(10368.0) - F::new(5.0) / F::new(1728.0) * t1227 * t4582 * t15453 * t70339 - F::new(5.0) / F::new(1728.0) * t3577 * t45128 * t1735 * t18206 + t52836 * t19077 / F::new(1024.0) - F::new(5.0) / F::new(648.0) * t66324 - t72936 / F::new(1152.0);
    t72938
}
