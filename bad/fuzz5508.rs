
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5508(_: S2, _: S6, _: S7) {}
        
        fn test5508() { foo5508(S4, S1, S5, S5, S6, S1); }
    