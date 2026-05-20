//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1674/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1674<F: Float>(t19862: F, t19899: F, t19939: F, t20007: F, t553: F, t5287: F, t5335: F, t1352: F, t19739: F, t1332: F, t1336: F, t1381: F, t1383: F, t16060: F, t1814: F, t1838: F, t1840: F, t19756: F, t19761: F, t19763: F, t19805: F, t19810: F, t19813: F, t19815: F, t5230: F, t5234: F, t5339: F, t5341: F, t5344: F, t5345: F, t5351: F, t544: F, t564: F, t6378: F, t6458: F) -> (F, F, F, F) {
    let t20009 = t19862 + t19899 + t19939 + t20007;
    let t20010 = t553 * t20009;
    let t20014 = t5335 * t5287;
    let t20018 = t19739 * t1352;
    let t20021 = t1332 * t6458 - F::new(2.0) * t1336 * t19756 - t1336 * t19813 - t1381 * t19815 + t1383 * t6378 - F::new(2.0) * t16060 * t1838 + F::new(2.0) * t1814 * t5351 + F::new(2.0) * t1840 * t5230 - t19761 * t5344 - t19763 * t5344 + t19805 * t564 - F::new(2.0) * t19810 * t5345 + t20010 * t544 - F::new(2.0) * t20014 * t5344 - F::new(2.0) * t20018 * t5344 - F::new(2.0) * t5234 * t5339 - F::new(2.0) * t5234 * t5341;
    (t20009, t20014, t20018, t20021)
}
