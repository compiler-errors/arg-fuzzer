
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15222(_: S7, _: S3, _: S7, _: S2, _: S6) {}
        
        fn test15222() { foo15222(S4, S3, S2, S1, S7, S1); }
    