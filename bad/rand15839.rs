
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15839(_: S7, _: S1) {}
        
        fn test15839() { foo15839(S2, S3, S4, S5, S6, S7, S8); }
    