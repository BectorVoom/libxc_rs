//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1466/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1466<F: Float>(t120865: F, t120867: F, t120869: F, t120871: F, t122765: F, t23880: F, t27273: F, t27276: F, t31284: F, t33195: F, t577: F, t7235: F, t7956: F, t8508: F, t86647: F, t96351: F) -> F {
    let t122774 = F::cast_from(27.0_f64) * t86647 * t7235 + t120865 + t120867 + t31284 + t8508 + t120869 + t120871 + F::cast_from(0.45e1_f64) * t122765 * t577 + F::cast_from(27.0_f64) * t96351 * t7956 + F::cast_from(27.0_f64) * t23880 * t27273 + F::cast_from(27.0_f64) * t23880 * t27276 + t33195;
    t122774
}
