
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13924(_: S1, _: S2, _: S3) {}
        
        fn test13924() { foo13924(S0, S7, S2, S0); }
    