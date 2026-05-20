//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2582/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582<F: Float>(t50853: F, t43768: F, t43770: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t44466: F, t50824: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F, t50881: F, t50886: F) -> F {
    let t52313 = F::new(5.0) / F::new(9.0) * t50853;
    let t52327 = -F::new(3.0) * t50824 + F::new(40.0) / F::new(81.0) * t50846 + t50848 / F::new(3.0) - t50851 / F::new(6.0) - t52313 - t43768 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t43770 - t44466 + t50859 / F::new(18.0) + F::new(2.0) * t50863 - t50867 - F::new(3.0) * t50871 - t50875 / F::new(3.0) - F::new(4.0) * t50881 + t50886 / F::new(6.0) - F::new(2.0) / F::new(9.0) * t43835 + F::new(2.0) / F::new(3.0) * t43837 + t43839 / F::new(9.0) + F::new(5.0) / F::new(27.0) * t43855 + F::new(4.0) / F::new(81.0) * t43857;
    t52327
}
