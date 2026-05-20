//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1300/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1300<F: Float>(t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F) -> (F, F) {
    let t42245 = F::cast_from(0.17757530864197530864e0_f64) * t41654;
    let t42253 = F::new(0.10274e0) * t41642 + F::cast_from(0.13698666666666666667e0_f64) * t41646 + F::cast_from(0.41095999999999999998e0_f64) * t41651 + t42245 - F::cast_from(0.45662222222222222221e-1_f64) * t41656 - F::cast_from(0.3044148148148148148e-1_f64) * t41658 + F::cast_from(0.25367901234567901233e-1_f64) * t41660 + F::cast_from(0.22831111111111111111e-1_f64) * t41662 - F::cast_from(0.50735802469135802467e-1_f64) * t41669 - F::cast_from(0.17123333333333333333e-1_f64) * t41673 + F::cast_from(0.9132444444444444444e-1_f64) * t41675;
    let t42266 = -F::cast_from(0.9132444444444444444e-1_f64) * t41678 + F::cast_from(0.4566222222222222222e-1_f64) * t41680 + F::cast_from(0.13698666666666666667e0_f64) * t41682 + F::cast_from(0.71030123456790123454e-1_f64) * t41684 + F::cast_from(0.2283111111111111111e0_f64) * t41690 - F::cast_from(0.11415555555555555555e0_f64) * t41695 - F::cast_from(0.41095999999999999999e0_f64) * t41699 - F::cast_from(0.34246666666666666665e-1_f64) * t41703 - F::cast_from(0.4566222222222222222e-1_f64) * t41707 + F::new(0.41096e0) * t41711 - F::cast_from(0.13698666666666666667e0_f64) * t41713 - F::new(0.61644e0) * t41717;
    (t42253, t42266)
}
