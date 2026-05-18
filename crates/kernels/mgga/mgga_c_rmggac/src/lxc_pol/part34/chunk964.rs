//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 964/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk964<F: Float>(t74503: F, t15523: F, t2191: F, t1986: F, t675: F, t9566: F, t68660: F, t68614: F, t74495: F, t74506: F, t74508: F, t74511: F, t74514: F, t74517: F, t77043: F, t77046: F, t77049: F, t77052: F, t77054: F, t77055: F) -> F {
    let t77056 = F::new(0.1276937996798935182e-4) * t74503;
    let t77057 = t2191 * t15523;
    let t77058 = F::new(0.42564599893297839398e-5) * t77057;
    let t77060 = t675 * t1986 * t9566;
    let t77061 = F::new(0.42564599893297839398e-5) * t77060;
    let t77062 = F::new(0.638468998399467591e-4) * t68660;
    let t77067 = -F::new(0.40992351065071538965e-4) * t68614 - t77043 + t77046 - t77049 - t77052 - F::new(0.13139479569676025391e-5) * t74495 + t77054 - t77055 + t77056 - t77058 - t77061 + t77062 + t74506 - F::new(0.3252672799280962148e-5) * t74508 - F::new(0.3252672799280962148e-5) * t74511 - F::new(0.3252672799280962148e-5) * t74514 - F::new(0.3252672799280962148e-5) * t74517;
    t77067
}
