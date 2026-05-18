//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 952/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk952<F: Float>(t74835: F, t74839: F, t74858: F, t74861: F, t74864: F, t74867: F, t74873: F, t74891: F, t74896: F, t74901: F, t74903: F, t74909: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77236 = F::new(0.69805943825008456614e-4) * t74835;
    let t77237 = F::new(0.11634323970834742769e-3) * t74839;
    let t77242 = F::new(0.1276937996798935182e-4) * t74858;
    let t77243 = F::new(0.1276937996798935182e-4) * t74861;
    let t77244 = F::new(0.638468998399467591e-4) * t74864;
    let t77246 = F::new(0.638468998399467591e-4) * t74867;
    let t77247 = F::new(0.81823984962736025184e-1) * t74873;
    let t77249 = F::new(0.85129199786595678799e-5) * t74891;
    let t77250 = F::new(0.85129199786595678799e-5) * t74896;
    let t77251 = F::new(0.85129199786595678799e-5) * t74901;
    let t77252 = F::new(0.2553875993597870364e-4) * t74903;
    let t77253 = F::new(0.2553875993597870364e-4) * t74909;
    (t77236, t77237, t77242, t77243, t77244, t77246, t77247, t77249, t77250, t77251, t77252, t77253)
}
