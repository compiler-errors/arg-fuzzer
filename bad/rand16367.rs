
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16367(_: S4, _: S5) {}
        
        fn test16367() { foo16367(S1, S2, S3, S4, S6); }
    