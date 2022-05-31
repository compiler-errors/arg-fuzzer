
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1172(_: S2, _: S3, _: S7, _: S8) {}
        
        fn test1172() { foo1172(S6, S3, S4, S4, S7, S0); }
    