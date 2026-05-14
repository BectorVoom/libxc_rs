//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 818/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk818<F: Float>(t1807: F, t8788: F, t32139: F, t32141: F, t32145: F, t32712: F, t32715: F, t32718: F, t32722: F, t32724: F, t539: F, t1375: F, t32127: F, t32154: F, t33241: F, t33247: F, t33251: F, t33274: F, t33298: F, t33798: F, t33804: F, t33810: F, t568: F, t7194: F, t7925: F) -> (F, F, F, F) {
    let t33815 = t1807 * t8788;
    let t33822 = -t32139 - 0.19378922925187387609e-1 * t32712 - t32141 - 0.32298204875312312682e-2 * t32715 + t32718 / 384.0 - t32722 / 384.0 - t32145 - t32724 / 96.0;
    let t33823 = t539 * t33822;
    let t33825 = -0.3289868133696452873e-1 * t33241 + 2.0 * t1375 * t33798 + 0.3289868133696452873e-1 * t33247 + 0.6579736267392905746e-1 * t33251 - t32127 + t32154 + 4.0 * t1375 * t33804 + 0.6579736267392905746e-1 * t33274 - 0.3289868133696452873e-1 * t33298 - 6.0 * t1375 * t33810 + 4.0 * t7194 * t7925 + t33815 * t568 + t33823 * t568;
    (t33815, t33822, t33823, t33825)
}
