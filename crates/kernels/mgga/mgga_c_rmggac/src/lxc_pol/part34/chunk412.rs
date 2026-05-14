//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 412/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk412<F: Float>(t2124: F, t551: F, t570: F, t7778: F, t305: F, t2064: F, t793: F, t2295: F, t6444: F, t1587: F, t645: F, t558: F, t797: F, t5271: F, t8625: F, t5162: F, t8631: F) -> (F, F, F, F, F, F, F, F) {
    let t8994 = t2124 * t551;
    let t8997 = t7778 * t570;
    let t8998 = t305 * t8997;
    let t9000 = t2064 * t551;
    let t9001 = t793 * t9000;
    let t9003 = t6444 * t2295;
    let t9005 = t645 * t1587;
    let t9006 = t793 * t9005;
    let t9008 = t2064 * t558;
    let t9009 = t797 * t9008;
    let t9011 = t5271 * t8625;
    let t9013 = t5162 * t8631;
    (t8994, t8998, t9001, t9003, t9006, t9009, t9011, t9013)
}
