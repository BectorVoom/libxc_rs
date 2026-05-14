//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 658/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk658<F: Float>(t321: F, t5259: F, t71949: F, t352: F, t5148: F, t22: F, t699: F, t3814: F, t3191: F, t7561: F, t2211: F, t838: F, t797: F, t333: F, t4669: F, t305: F, t71637: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71951 = t5259 * t71949 * t321;
    let t71960 = t5148 * t71949 * t352;
    let t71982 = t699 * t22;
    let t71983 = t3814 * t71982;
    let t72010 = t3191 * t7561;
    let t72011 = 0.33335697577410973224e-1 * t72010;
    let t72019 = t2211 * t22;
    let t72020 = t838 * t72019;
    let t72023 = t797 * t72019;
    let t72027 = t4669 * t71949 * t333;
    let t72037 = t305 * t71637;
    (t71951, t71960, t71982, t71983, t72011, t72020, t72023, t72027, t72037)
}
