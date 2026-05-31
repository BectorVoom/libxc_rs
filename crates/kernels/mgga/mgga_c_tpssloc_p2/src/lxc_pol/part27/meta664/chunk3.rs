//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2333/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2333<F: Float>(t16311: F, t3788: F, t3791: F, t6936: F, t80784: F, t80792: F, t80794: F, t1339: F, t1825: F, t26288: F, t3734: F, t80780: F, t80789: F, t80796: F, t80801: F, t80807: F, t80814: F, t80821: F, t80826: F, t80828: F, t91226: F, t91229: F, t91233: F, t91237: F) -> F {
    let t91241 = t6936 * t3788 * t16311 * t3791;
    let t91244 = F::cast_from(0.33643963411783659044e-4_f64) * t80784;
    let t91246 = F::cast_from(0.10541775202358879834e-2_f64) * t80792;
    let t91247 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t80794;
    let t91256 = t26288 * t1339 * t1825 * t3734;
    let t91258 = t91226 - F::cast_from(0.20186378047070195427e-3_f64) * t91229 - F::cast_from(0.24223653656484234512e-2_f64) * t91233 - F::cast_from(0.12111826828242117256e-2_f64) * t91237 + F::cast_from(0.12111826828242117256e-2_f64) * t91241 - F::cast_from(0.63250651214153279005e-2_f64) * t80780 + t91244 + F::cast_from(0.33643963411783659045e-4_f64) * t80789 - t91246 + t91247 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t80796 - F::cast_from(0.6728792682356731809e-4_f64) * t80801 + F::cast_from(0.33643963411783659045e-4_f64) * t80807 + F::cast_from(0.20186378047070195427e-3_f64) * t80814 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t80821 - t80826 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t80828 - F::cast_from(0.84782787797694820792e-2_f64) * t91256;
    t91258
}
