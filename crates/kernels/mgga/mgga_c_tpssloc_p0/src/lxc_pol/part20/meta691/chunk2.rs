//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2625/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2625<F: Float>(t11835: F, t4889: F, t1174: F, t1725: F, t2402: F, t11665: F, t11668: F, t11692: F, t11845: F, t11850: F, t1227: F, t14730: F, t14748: F, t15654: F, t15708: F, t15710: F, t3577: F, t3578: F, t45250: F, t4582: F, t4723: F, t48554: F, t52532: F, t52538: F, t53144: F) -> F {
    let t53433 = t4889 * t11835;
    let t53434 = t53433 / F::new(162.0);
    let t53440 = t1174 * t2402 * t1725;
    let t53446 = F::new(5.0) / F::new(768.0) * t3577 * t11668 * t14730 * t52538 - F::new(5.0) / F::new(4608.0) * t11692 * t11668 * t4723 * t53144 - t11665 * t15710 / F::new(384.0) - t3577 * t3578 * t14748 * t15708 / F::new(384.0) + F::new(5.0) / F::new(768.0) * t1227 * t4582 * t15654 * t48554 - t45250 - t53434 + t4889 * t11845 / F::new(108.0) + t4889 * t11850 / F::new(18.0) - F::new(5.0) / F::new(3888.0) * t53440 + F::new(5.0) / F::new(4608.0) * t3577 * t11668 * t4723 * t52532;
    t53446
}
