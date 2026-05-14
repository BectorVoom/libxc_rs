//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1015/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1015<F: Float>(t33755: F, t34371: F, t1458: F, t32609: F, t33148: F, t33150: F, t33152: F, t33154: F, t33711: F, t33713: F, t33715: F, t34229: F, t8446: F, t113: F, t1459: F, t1774: F, t1849: F, t2114: F, t2165: F, t33096: F, t33098: F, t33100: F, t33131: F, t33139: F, t33158: F, t33162: F, t33736: F, t33747: F, t33748: F, t33758: F, t510: F, t574: F, t7983: F, t8103: F, t8860: F, t8916: F) -> (F, F, F) {
    let t34372 = t33755 + t34371;
    let t34381 = 2.0 * t1458 * t32609 + t33148 + t33150 + t33152 + t33154 + 4.0 * t33711 + 4.0 * t33713 + 4.0 * t33715 + t34229 + t8446;
    let t34384 = -t113 * t34372 - 2.0 * t1459 * t32609 - t1774 * t8860 + t1849 * t8916 - 2.0 * t2114 * t8103 - 2.0 * t2165 * t7983 - t34229 * t510 + t34381 * t574 - t33096 - t33098 - t33100 + t33131 - t33139 - t33158 - t33162 - 4.0 * t33736 + 2.0 * t33747 + 6.0 * t33748 + 2.0 * t33758;
    (t34372, t34381, t34384)
}
