//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1027/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1027<F: Float>(t25: F, t265: F, t394: F, t123798: F, t123835: F, t123428: F, t123766: F, t1409: F, t32072: F, t34031: F, t3966: F, t40: F, t607: F, t8760: F, t116473: F, t116476: F, t116481: F, t119755: F, t119763: F, t123378: F, t123382: F, t123398: F, t123414: F, t123715: F, t123733: F, t123752: F, t1649: F, t1877: F, t23788: F, t24191: F, t2522: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t25927: F, t25930: F, t25934: F, t25938: F, t26756: F, t28: F, t32030: F, t32034: F, t7109: F, t7114: F, t7656: F, t8748: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t123836 = t123798 + t123835;
    let t123837 = piecewise3::<F>(t395, F::new(0.0), t123836);
    let t123844 = piecewise3::<F>(t115, t123428 + t123766, t123837 * t40 / F::new(2.0) + t32072 * t1409 / F::new(2.0) + t34031 * t607 / F::new(2.0) + t8760 * t3966 / F::new(2.0));
    let t123888 = t1877 * t123715 * t28 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t116473 * t25898 - t123398 - t1877 * t32034 * t25930 / F::new(2.0) + F::new(3.0) * t123382 * t25892 - F::new(3.0) * t24191 * t23788 * t123414 - F::new(3.0) / F::new(2.0) * t116473 * t25921 + t1877 * t32030 * t1649 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8748 * t25938 + F::new(3.0) * t116481 * t119763 - t1877 * t116476 * t7656 / F::new(2.0) + t123733 - t1877 * t7114 * t1649 * t7109 - t1877 * t32034 * t25934 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8748 * t25901 - F::new(3.0) * t123378 * t119755 + F::new(2.0) * t26756 * t25927 * t123752 - F::new(3.0) / F::new(2.0) * t2522 * t8748 * t25905;
    (t123836, t123844, t123888)
}
