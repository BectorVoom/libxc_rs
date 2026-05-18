//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 774/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk774<F: Float>(t1981: F, t676: F, t687: F, t8512: F, t15361: F, t495: F, t14230: F, t14237: F, t2067: F, t14225: F, t3352: F, t8496: F) -> (F, F, F) {
    let t74003 = t8512 * t1981 * t676 * t687;
    let t74005 = t15361 * t495;
    let t74008 = t14230 * t14237 * t2067 * t74005;
    let t74013 = t14225 * t3352 * t8496;
    (t74003, t74008, t74013)
}
