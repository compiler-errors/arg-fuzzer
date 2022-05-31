
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10887(_: S3, _: S4, _: S5, _: S8) {}
        
        fn test10887() { foo10887(S1, S2, S3, S4, S6, S7, S8); }
    