//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2603/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2603<F: Float>(t11651: F, t15507: F, t11709: F, t1174: F, t11741: F, t1177: F, t11805: F, t11809: F, t15622: F, t15627: F, t15631: F, t1737: F, t3506: F, t44858: F, t44896: F, t45080: F, t4582: F, t4978: F, t5005: F, t50865: F, t50869: F, t52659: F, t52836: F) -> F {
    let t52845 = t15507 * t11651;
    let t52853 = t11709 * t15622 / F::new(512.0) + F::new(3.0) / F::new(512.0) * t44896 * t15627 - F::new(3.0) / F::new(512.0) * t44858 * t15631 + t3506 * t4582 * t52659 * t4978 / F::new(512.0) + t52836 * t11741 / F::new(3072.0) - t5005 * t11805 / F::new(4608.0) - t5005 * t11809 / F::new(768.0) + t45080 * t1737 / F::new(3072.0) + t52845 / F::new(288.0) - t1174 * t1177 * t50865 / F::new(48.0) - t1174 * t1177 * t50869 / F::new(16.0);
    t52853
}
