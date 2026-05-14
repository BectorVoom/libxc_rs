//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1334/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1334<F: Float>(t2056: F, t20957: F, t22117: F, t3499: F, t3542: F, t5314: F, t5984: F, t68835: F, t68837: F, t68841: F, t68843: F, t68845: F, t68848: F, t68850: F, t68853: F, t68857: F, t68859: F, t68861: F, t68863: F, t68865: F, t68867: F, t68870: F) -> (F,) {
    let t72798 = -2.0 * t2056 * t22117 - 4.0 * t20957 * t3542 - 2.0 * t22117 * t3499 - t5314 * t5984 + t68835 + t68837 - t68841 - t68843 - t68845 - t68848 - t68850 - t68853 + t68857 - t68859 - t68861 - t68863 + t68865 - t68867 + t68870;
    (t72798,)
}
