
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8005(_: S0, _: S2, _: S7, _: S6, _: S3, _: S7, _: S1) {}
        
        fn test8005() { foo8005(S1, S2, S3, S4, S5, S6, S7, S8); }
    