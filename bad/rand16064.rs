
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16064(_: S4, _: S7) {}
        
        fn test16064() { foo16064(S1, S3, S4, S5, S8); }
    