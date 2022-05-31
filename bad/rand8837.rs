
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8837(_: S1, _: S3) {}
        
        fn test8837() { foo8837(S2, S5, S1, S5, S4); }
    