
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1410(_: S1, _: S3, _: S5) {}
        
        fn test1410() { foo1410(S5, S2, S5, S1, S6, S5); }
    