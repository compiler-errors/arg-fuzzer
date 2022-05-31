
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo565(_: S3, _: S2, _: S6, _: S4, _: S1) {}
        
        fn test565() { foo565(S7, S1, S2, S4, S1, S1, S1); }
    