
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16575(_: S4, _: S7) {}
        
        fn test16575() { foo16575(S2, S4, S5, S6, S8); }
    