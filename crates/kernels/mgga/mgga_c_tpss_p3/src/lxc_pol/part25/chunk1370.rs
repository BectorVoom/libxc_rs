//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1370/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1370<F: Float>(t1692: F, t1812: F, t18728: F, t18812: F, t20021: F, t20050: F, t20065: F, t20514: F, t20526: F, t21492: F, t21499: F, t2439: F, t5849: F, t5853: F, t62610: F, t6354: F, t70805: F, t70808: F, t70816: F, t70828: F, t70839: F, t70857: F, t70887: F, t70909: F, t70960: F) -> F {
    let t72460 = F::new(3.0) / F::new(2.0) * t2439 * t1812 * t70857 + F::new(2.0) * t20526 * t70805 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t70839 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t70909 + F::new(3.0) * t2439 * t6354 * t20021 - F::new(3.0) * t20526 * t70828 + F::new(3.0) / F::new(2.0) * t2439 * t5849 * t21499 - t1692 * t5853 * t70960 + t1692 * t18812 * t70808 - t1692 * t5853 * t70816 / F::new(2.0) - F::new(3.0) * t18728 * t70887 - t1692 * t20514 * t20050 - t1692 * t20514 * t20065 - F::new(3.0) * t62610 * t21492;
    t72460
}
