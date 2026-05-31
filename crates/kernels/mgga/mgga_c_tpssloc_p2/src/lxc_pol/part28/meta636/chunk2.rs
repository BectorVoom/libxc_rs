//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2022/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2022<F: Float>(t91281: F, t91283: F, t91286: F, t91290: F, t91300: F, t80837: F, t80843: F, t80857: F, t80859: F, t84520: F, t91261: F, t91263: F, t91268: F, t91272: F, t91276: F, t91279: F, t91294: F, t91298: F) -> F {
    let t93710 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91281;
    let t93711 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91283;
    let t93712 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91286;
    let t93715 = F::cast_from(0.33913115119077928316e-1_f64) * t91290;
    let t93718 = F::cast_from(0.11304371706359309439e-1_f64) * t91300;
    let t93719 = -t91261 / F::cast_from(48.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t91263 + F::cast_from(0.40372756094140390852e-3_f64) * t80837 - F::cast_from(0.28260929265898273597e-2_f64) * t80843 - t84520 - F::cast_from(0.80745512188280781706e-3_f64) * t91268 + F::cast_from(0.48447307312968469024e-2_f64) * t91272 + F::cast_from(0.24223653656484234512e-2_f64) * t91276 - t91279 / F::cast_from(384.0_f64) + t93710 + t93711 + t93712 - F::cast_from(0.80745512188280781706e-3_f64) * t80857 - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t80859 - t93715 + F::cast_from(0.48447307312968469024e-2_f64) * t91294 + F::cast_from(0.24223653656484234512e-2_f64) * t91298 - t93718;
    t93719
}
