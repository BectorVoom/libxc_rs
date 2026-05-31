//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2781/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781<F: Float>(t17013: F, t9638: F, t13258: F, t16845: F, t13261: F, t4166: F, t13151: F, t13156: F, t13164: F, t13191: F, t16723: F, t16729: F, t16737: F, t16749: F, t1891: F, t228: F, t2379: F, t2667: F, t2671: F, t2675: F, t4219: F, t4225: F, t4227: F, t4230: F, t5544: F, t5601: F, t5605: F, t5608: F, t58090: F, t58139: F, t68: F, t822: F, t824: F, t825: F) -> (F, F, F, F) {
    let t58890 = t9638 * t17013;
    let t58900 = t13258 * t16845;
    let t58904 = t4166 * t13261;
    let t58947 = F::cast_from(60.0_f64) * t1891 * t2379 * t4225 * t5544 + F::cast_from(240.0_f64) * t13156 * t13191 * t4225 - F::cast_from(24.0_f64) * t228 * t2671 * t58090 + F::cast_from(3.0_f64) * t228 * t58139 * t824 - F::cast_from(48.0_f64) * t4219 * t4227 * t68 + F::cast_from(120.0_f64) * t13151 * t16737 - F::cast_from(24.0_f64) * t13164 * t16729 + F::cast_from(6.0_f64) * t16723 * t825 + F::cast_from(6.0_f64) * t16749 * t822 - F::cast_from(12.0_f64) * t2667 * t5605 + F::cast_from(3.0_f64) * t2667 * t5608 + F::cast_from(3.0_f64) * t2675 * t5601 + F::cast_from(12.0_f64) * t4219 * t4230;
    (t58890, t58900, t58904, t58947)
}
