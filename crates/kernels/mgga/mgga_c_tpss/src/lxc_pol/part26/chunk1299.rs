//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1299/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1299<F: Float>(t69509: F, t69529: F, t69550: F, t69567: F, t13884: F, t13889: F, t13941: F, t1639: F, t1772: F, t1773: F, t1775: F, t18483: F, t18490: F, t19497: F, t19500: F, t19507: F, t19509: F, t19517: F, t19527: F, t19552: F, t19559: F, t21070: F, t21093: F, t4494: F, t520: F, t522: F, t5432: F, t5448: F, t5731: F, t5734: F, t5739: F, t5740: F, t5742: F, t5745: F, t5748: F, t6268: F, t6271: F, t65667: F, t69452: F, t69458: F) -> (F, F) {
    let t69569 = t69509 + t69529 + t69550 + t69567;
    let t69575 = -t5734 * t13941 + 4.0 * t5734 * t13884 - t69452 * t1775 + 2.0 * t5734 * t13889 + 4.0 * t19500 * t4494 + 2.0 * t69458 * t5742 + 2.0 * t18483 * t21093 + 4.0 * t19509 * t19527 - 12.0 * t19509 * t19517 + 2.0 * t65667 * t6268 + t69458 * t5748 + 4.0 * t19509 * t19559 - 6.0 * t18483 * t21070 + 2.0 * t5739 * t5740 * t5731 * t5448 - 2.0 * t19507 * t6271 - 6.0 * t5739 * t18490 * t5731 * t5432 + 2.0 * t5739 * t5745 * t19497 * t1639 * t520 - t1772 * t1773 * t522 * t69569 + 2.0 * t19509 * t19552;
    (t69569, t69575)
}
