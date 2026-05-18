//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1234/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1234<F: Float>(t28: F, t265: F, t504: F, t23788: F, t33476: F, t25927: F, t33483: F, t1649: F, t1914: F, t33512: F, t1409: F, t1877: F, t24191: F, t2522: F, t26744: F, t26756: F, t31434: F, t33065: F, t33466: F, t52: F, t7114: F, t7649: F, t7656: F, t8566: F, t8586: F, t8591: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t33531 = t23788 * t33476;
    let t33537 = t25927 * t33483;
    let t33539 = t1649 * t1914;
    let t33547 = piecewise3::<f64>(t505, F::new(0.0), t33512);
    let t33552 = piecewise3::<f64>(t401, F::new(3.0) / F::new(2.0) * t2522 * t8566 * t7649 + t1877 * t33466 * t28 / F::new(2.0) - t1877 * t31434 * t7656 / F::new(2.0) + t1877 * t8566 * t1649 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t33531 - t1877 * t26744 * t8586 / F::new(2.0) + t26756 * t33537 - t1877 * t7114 * t33539 / F::new(2.0) - t1877 * t7114 * t33065 / F::new(2.0), -t8591 * t1409 / F::new(2.0) + t33547 * t52 / F::new(2.0));
    (t33531, t33537, t33539, t33547, t33552)
}
