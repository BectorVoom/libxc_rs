//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3193/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193<F: Float>(t18363: F, t3577: F, t45124: F, t11697: F, t18359: F, t15459: F, t15463: F, t15478: F, t15631: F, t15740: F, t18321: F, t18368: F, t3562: F, t45044: F, t45049: F, t45162: F, t53135: F, t53142: F, t53155: F, t53158: F, t53161: F, t53185: F, t53472: F) -> F {
    let t66334 = t3577 * t45124 * t18363;
    let t66337 = t3577 * t11697 * t18359;
    let t66353 = -t15740 * t15478 / F::new(1152.0) - t15740 * t15459 / F::new(2304.0) - t15740 * t15463 / F::new(1152.0) + F::new(5.0) / F::new(10368.0) * t66334 - t66337 / F::new(1728.0) - F::new(5.0) / F::new(1944.0) * t45044 + t53135 / F::new(1728.0) - F::new(5.0) / F::new(62208.0) * t45049 - t53472 * t15631 / F::new(256.0) + F::new(11.0) / F::new(243.0) * t18321 * t3562 - t53142 / F::new(432.0) - t45162 * t18368 / F::new(1152.0) - t53155 / F::new(3456.0) - t53158 / F::new(1728.0) + t53161 / F::new(5184.0) + t53185 / F::new(2304.0);
    t66353
}
