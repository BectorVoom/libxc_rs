//! GGA_X_2D_B86_MGC lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b86_mgc_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t25 = 1.0 + 0.16646e-1 * t23;
        let t26 = pow_1_4(t25);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t29 = 1.0 / t28;
        let t32 = 1.0 + 0.4409422067590197497e-2 * t23 * t29;
        let t36 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t38 = t17 / t18;
        let t42 = t20 * t20;
        let t43 = 1.0 / t42;
        let t47 = sigma[ip] * sigma[ip];
        let t48 = t42 * t21;
        let t49 = 1.0 / t48;
        let t52 = 1.0 / t28 / t25;
        let t55 = -0.13228266202770592491e-1 * sigma[ip] * t43 * t29 + 0.16514828940848946195e-3 * t47 * t49 * t52;
        let t60 = piecewise3(t2, 0.0, -t16 * t38 * t32 / 3.0 - 2.0 / 3.0 * t16 * t19 * t55);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t65 = t42 * t20;
        let t66 = 1.0 / t65;
        let t67 = sigma[ip] * t66;
        let t70 = 0.4409422067590197497e-2 * t22 * t29 - 0.55049429802829820651e-4 * t67 * t52;
        let t74 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t70);
        let tvsigma0 = 2.0 * rho[ip] * t74;
        vsigma[ip] += tvsigma0;
        let t79 = t17 / t18 / rho[ip];
        let t86 = t42 * rho[ip];
        let t87 = 1.0 / t86;
        let t91 = t42 * t42;
        let t92 = 1.0 / t91;
        let t96 = t47 * sigma[ip];
        let t98 = 1.0 / t91 / t21;
        let t100 = t25 * t25;
        let t102 = 1.0 / t28 / t100;
        let t105 = 0.52913064811082369964e-1 * sigma[ip] * t87 * t29 - 0.16514828940848946195e-2 * t47 * t92 * t52 + 0.14432556733842006814e-4 * t96 * t98 * t102;
        let t110 = piecewise3(t2, 0.0, t16 * t79 * t32 / 6.0 - 2.0 / 3.0 * t16 * t38 * t55 - 2.0 / 3.0 * t16 * t19 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t110 + 4.0 * t60;
        v2rho2[ip] += tv2rho20;
        let t118 = t49 * t52;
        let t122 = 1.0 / t91 / t20;
        let t123 = t47 * t122;
        let t126 = -0.13228266202770592491e-1 * t43 * t29 + 0.49544486822546838586e-3 * t118 * sigma[ip] - 0.48108522446140022714e-5 * t123 * t102;
        let t131 = piecewise3(t2, 0.0, -t16 * t38 * t70 / 3.0 - 2.0 / 3.0 * t16 * t19 * t126);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t74;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = 1.0 / t91 / rho[ip];
        let t141 = -0.1100988596056596413e-3 * t66 * t52 + 0.16036174148713340905e-5 * sigma[ip] * t137 * t102;
        let t145 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t141);
        let tv2sigma20 = 2.0 * rho[ip] * t145;
        v2sigma2[ip] += tv2sigma20;
        let t150 = t17 / t18 / t20;
        let t165 = 1.0 / t91 / t42;
        let t169 = t47 * t47;
        let t171 = 1.0 / t91 / t48;
        let t175 = 1.0 / t28 / t100 / t25;
        let t178 = -0.26456532405541184982e0 * t67 * t29 + 0.15193642625581030499e-1 * t47 * t137 * t52 - 0.30308369141068214309e-3 * t96 * t165 * t102 + 0.19820157999801558748e-5 * t169 * t171 * t175;
        let t183 = piecewise3(t2, 0.0, -t16 * t150 * t32 / 4.0 + t16 * t79 * t55 / 2.0 - t16 * t38 * t105 - 2.0 / 3.0 * t16 * t19 * t178);
        let tv3rho30 = 2.0 * rho[ip] * t183 + 6.0 * t110;
        v3rho3[ip] += tv3rho30;
        let t195 = t92 * t52;
        let t198 = t98 * t102;
        let t202 = 1.0 / t91 / t65;
        let t206 = 0.52913064811082369964e-1 * t87 * t29 - 0.39635589458037470869e-2 * t195 * sigma[ip] + 0.91406192647666043157e-4 * t198 * t47 - 0.66067193332671862493e-6 * t96 * t202 * t175;
        let t211 = piecewise3(t2, 0.0, t16 * t79 * t70 / 6.0 - 2.0 / 3.0 * t16 * t38 * t126 - 2.0 / 3.0 * t16 * t19 * t206);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t211 + 4.0 * t131;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t218 = t122 * t102;
        let t222 = 1.0 / t91 / t86;
        let t226 = 0.6605931576339578478e-3 * t118 - 0.24054261223070011357e-4 * t218 * sigma[ip] + 0.22022397777557287498e-6 * t47 * t222 * t175;
        let t231 = piecewise3(t2, 0.0, -t16 * t38 * t141 / 3.0 - 2.0 / 3.0 * t16 * t19 * t226);
        let tv3rhosigma20 = 2.0 * rho[ip] * t231 + 2.0 * t145;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t239 = 0.48108522446140022714e-5 * t137 * t102 - 0.73407992591857624994e-7 * sigma[ip] * t165 * t175;
        let t243 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t239);
        let tv3sigma30 = 2.0 * rho[ip] * t243;
        v3sigma3[ip] += tv3sigma30;
        let t267 = t91 * t91;
        let t268 = 1.0 / t267;
        let t276 = t100 * t100;
        let t278 = 1.0 / t28 / t276;
        let t286 = piecewise3(t2, 0.0, 5.0 / 8.0 * t16 * t17 / t18 / t21 * t32 - t16 * t150 * t55 + t16 * t79 * t105 - 4.0 / 3.0 * t16 * t38 * t178 - 2.0 / 3.0 * t16 * t19 * (0.15873919443324710989e1 * sigma[ip] * t49 * t29 - 0.14665168099473864221e0 * t123 * t52 + 0.4964799516441650344e-2 * t96 * t222 * t102 - 0.71352568799285611491e-4 * t169 * t268 * t175 + 0.37116714382278384028e-6 * t169 * sigma[ip] / t267 / t21 * t278));
        let tv4rho40 = 2.0 * rho[ip] * t286 + 8.0 * t183;
        v4rho4[ip] += tv4rho40;
        let t319 = piecewise3(t2, 0.0, -t16 * t150 * t70 / 4.0 + t16 * t79 * t126 / 2.0 - t16 * t38 * t206 - 2.0 / 3.0 * t16 * t19 * (-0.26456532405541184982e0 * t66 * t29 + 0.33690251039331850238e-1 * t137 * t52 * sigma[ip] - 0.13518494807365346382e-2 * t165 * t102 * t47 + 0.21802173799781714623e-4 * t171 * t175 * t96 - 0.12372238127426128009e-6 * t169 / t267 / t20 * t278));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t319 + 6.0 * t211;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t345 = piecewise3(t2, 0.0, t16 * t79 * t141 / 6.0 - 2.0 / 3.0 * t16 * t38 * t226 - 2.0 / 3.0 * t16 * t19 * (-0.46241521034377049346e-2 * t195 + 0.29827283916606814083e-3 * t198 * sigma[ip] - 0.61662713777160404994e-5 * t202 * t175 * t47 + 0.41240793758087093365e-7 * t96 / t267 / rho[ip] * t278));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t345 + 4.0 * t231;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t363 = piecewise3(t2, 0.0, -t16 * t38 * t239 / 3.0 - 2.0 / 3.0 * t16 * t19 * (-0.43297670201526020443e-4 * t218 + 0.15415678444290101249e-5 * t222 * t175 * sigma[ip] - 0.13746931252695697789e-7 * t47 * t268 * t278));
        let tv4rhosigma30 = 2.0 * rho[ip] * t363 + 2.0 * t243;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t375 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * (-0.29363197036743049997e-6 * t165 * t175 + 0.45823104175652325962e-8 * sigma[ip] * t171 * t278));
        let tv4sigma40 = 2.0 * rho[ip] * t375;
        v4sigma4[ip] += tv4sigma40;
    }
}
