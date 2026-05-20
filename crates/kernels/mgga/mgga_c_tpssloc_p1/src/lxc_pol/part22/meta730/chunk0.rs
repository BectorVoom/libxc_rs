//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2396/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2396<F: Float>(t21185: F, t41935: F, t896: F, t17210: F, t4370: F, t13629: F, t5705: F, t17271: F, t4362: F, t41942: F, t17218: F, t41962: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F) -> (F, F, F, F, F, F, F) {
    let t68619 = t41935 * t21185 * t896;
    let t68626 = t17210 * t4370;
    let t68628 = t13629 * t5705;
    let t68630 = t4362 * t17271;
    let t68633 = t41942 * t21185 * t896;
    let t68635 = t17218 * t4370;
    let t68637 = t41962 - F::cast_from(0.485484375e1_f64) * t68619 + F::new(0.5519e-1) * t60274 - F::new(0.12077e1) * t59700 + F::cast_from(0.40256666666666666666e0_f64) * t59702 + F::cast_from(0.33547222222222222222e0_f64) * t59704 + F::cast_from(0.93932222222222222225e0_f64) * t47787 + F::new(0.58258125e1) * t68626 - F::new(0.3883875e1) * t68628 - F::new(0.3883875e1) * t68630 + F::cast_from(0.6189328125e-1_f64) * t68633 - F::cast_from(0.1237865625e0_f64) * t68635;
    (t68619, t68626, t68628, t68630, t68633, t68635, t68637)
}
