
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9083(_: S3, _: S4) {}
        
        fn test9083() { foo9083(S1, S3, S6); }
    